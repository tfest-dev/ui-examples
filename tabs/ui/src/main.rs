use ::image::ImageFormat;
use iced::widget::{button, column, container, image, row, scrollable, text};
use iced::{Alignment, Color, Element, Length, Sandbox, Settings};
use pdfium::{set_library_location, PdfiumDocument, PdfiumRenderConfig, PdfiumResult};
use std::fs;
use tabs_backend::{AppState, BomItem, TabKind};

// Ink wash palette
fn charcoal() -> Color {
    Color::from_rgb8(0x4A, 0x4A, 0x4A)
}
fn cool_gray() -> Color {
    Color::from_rgb8(0xCB, 0xCB, 0xCB)
}
fn soft_ivory() -> Color {
    Color::from_rgb8(0xFF, 0xFF, 0xE3)
}
fn slate_blue() -> Color {
    Color::from_rgb8(0x6D, 0x81, 0x96)
}

// Approximate column widths for the BoM table (in logical px).
const COL_NAME_WIDTH: f32 = 260.0;
const COL_QTY_WIDTH: f32 = 60.0;
const COL_UNIT_WIDTH: f32 = 100.0;
const COL_TOTAL_WIDTH: f32 = 110.0;
const COL_LEAD_WIDTH: f32 = 100.0;
const COL_MIN_WIDTH: f32 = 80.0;

pub fn main() -> iced::Result {
    TabsApp::run(Settings::default())
}

fn render_quote_pdf_to_png(pdf_path: &str, output_png: &str) -> PdfiumResult<()> {
    // Ensure the output directory for the rendered preview exists.
    if let Some(parent) = std::path::Path::new(output_png).parent() {
        let _ = fs::create_dir_all(parent);
    }

    // Configure pdfium-rs to load the Pdfium shared library from the repo-local location.
    set_library_location("../rust/lib");

    let doc = PdfiumDocument::new_from_path(pdf_path, None)?;
    // Render at a higher resolution so text remains readable
    // when scaled into the preview area.
    let config = PdfiumRenderConfig::new().with_height(1200);
    let page = doc.page(0)?;
    let bitmap = page.render(&config)?;

    bitmap.save(output_png, ImageFormat::Png)?;
    Ok(())
}

struct TabsApp {
    backend_state: AppState,
    active_tab: TabKind,
    quote_image_path: Option<String>,
}

struct MaterialRow {
    name: &'static str,
    lead_time_days: u32,
    min_quantity: u32,
}

fn demo_materials() -> Vec<MaterialRow> {
    vec![
        MaterialRow {
            name: "Steel frame sections",
            lead_time_days: 21,
            min_quantity: 50,
        },
        MaterialRow {
            name: "Electrical fixtures",
            lead_time_days: 14,
            min_quantity: 40,
        },
        MaterialRow {
            name: "Finishing materials",
            lead_time_days: 10,
            min_quantity: 100,
        },
    ]
}

#[derive(Debug, Clone, Copy)]
enum Message {
    TabSelected(TabKind),
}

impl Sandbox for TabsApp {
    type Message = Message;

    fn new() -> Self {
        // BOM is wired from a CSV file under tabs/examples.
        let backend_state = AppState::demo_with_bom_path("../examples/bom.csv");

        // Render the example quote PDF into a PNG that the UI can display.
        let pdf_path = "../examples/quote.pdf";
        let png_path = "../gen/quote_preview.png";
        let quote_image_path = match render_quote_pdf_to_png(pdf_path, png_path) {
            Ok(()) => Some(png_path.to_string()),
            Err(err) => {
                eprintln!("PDF render error: {err:?}");
                None
            }
        };

        Self {
            backend_state,
            active_tab: TabKind::Overview,
            quote_image_path,
        }
    }

    fn title(&self) -> String {
        String::from("Tabs example – Rust + Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab) => {
                self.active_tab = tab;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let tabs_row = row![
            tab_button("Quote", TabKind::Overview, self.active_tab),
            tab_button("Breakdown", TabKind::Logs, self.active_tab),
            tab_button("Planning", TabKind::Advanced, self.active_tab),
            tab_button("Settings", TabKind::Settings, self.active_tab),
        ]
        .spacing(12);

        let header = column![
            text("Project estimate")
                .size(22)
                .style(iced::theme::Text::Color(soft_ivory())),
            text("Preview, breakdown, planning, and configuration in one view.")
                .size(14)
                .style(iced::theme::Text::Color(cool_gray())),
        ]
        .spacing(4);

        let content: Element<_> = match self.active_tab {
            // Quote tab: preview of the exported quote as a rendered PDF page.
            TabKind::Overview => {
                let pdf_path = "../examples/quote.pdf";

                let preview_content: Element<_> = if let Some(path) = &self.quote_image_path {
                    // Display the rendered PNG of the first page.
                    image::viewer(image::Handle::from_path(path)).into()
                } else {
                    // Fallback text if rendering failed.
                    column![
                        text("Quote PDF preview")
                            .size(16)
                            .style(iced::theme::Text::Color(charcoal())),
                        text(format!("Source: {}", pdf_path))
                            .size(12)
                            .style(iced::theme::Text::Color(cool_gray())),
                        text("This area would render a formatted PDF of the quote before you export or send it.")
                            .size(13)
                            .style(iced::theme::Text::Color(cool_gray())),
                    ]
                    .spacing(6)
                    .into()
                };

                let preview_box = container(preview_content)
                    .width(Length::Fill)
                    .height(Length::Fixed(600.0))
                    .padding(16)
                    .style(iced::theme::Container::Custom(Box::new(
                        |_t: &iced::Theme| {
                            use iced::widget::container;
                            container::Appearance {
                                background: Some(cool_gray().into()),
                                border: iced::Border {
                                    radius: 4.0.into(),
                                    width: 1.0,
                                    color: slate_blue(),
                                },
                                ..Default::default()
                            }
                        },
                    )));

                column![
                    text("Quote preview")
                        .size(18)
                        .style(iced::theme::Text::Color(slate_blue())),
                    preview_box,
                ]
                .spacing(12)
                .into()
            }
            // Breakdown tab: detailed BoQ / BoM-style list backed by the shared state.
            TabKind::Logs => {
                // Explicitly show which BoM source file is driving this view.
                let bom_path = "../examples/bom.csv";
                let materials: &[BomItem] = self.backend_state.bom();

                let header_row = row![
                    container(
                        text("Material")
                            .size(14)
                            .style(iced::theme::Text::Color(cool_gray()))
                    )
                    .width(Length::Fixed(COL_NAME_WIDTH)),
                    container(
                        text("Qty")
                            .size(14)
                            .style(iced::theme::Text::Color(cool_gray()))
                    )
                    .width(Length::Fixed(COL_QTY_WIDTH)),
                    container(
                        text("Unit cost")
                            .size(14)
                            .style(iced::theme::Text::Color(cool_gray()))
                    )
                    .width(Length::Fixed(COL_UNIT_WIDTH)),
                    container(
                        text("Line total")
                            .size(14)
                            .style(iced::theme::Text::Color(cool_gray()))
                    )
                    .width(Length::Fixed(COL_TOTAL_WIDTH)),
                    container(
                        text("Lead time")
                            .size(14)
                            .style(iced::theme::Text::Color(cool_gray()))
                    )
                    .width(Length::Fixed(COL_LEAD_WIDTH)),
                    container(
                        text("Min qty")
                            .size(14)
                            .style(iced::theme::Text::Color(cool_gray()))
                    )
                    .width(Length::Fixed(COL_MIN_WIDTH)),
                ]
                .spacing(8);

                let rows = materials.iter().fold(
                    column![
                        text(format!("BoM source: {}", bom_path))
                            .size(12)
                            .style(iced::theme::Text::Color(cool_gray())),
                        header_row,
                    ]
                    .spacing(6),
                    |col, m| {
                        col.push(
                            row![
                                container(
                                    text(&m.name)
                                        .size(14)
                                        .style(iced::theme::Text::Color(soft_ivory()))
                                )
                                .width(Length::Fixed(COL_NAME_WIDTH)),
                                container(
                                    text(m.quantity.to_string())
                                        .size(14)
                                        .style(iced::theme::Text::Color(soft_ivory()))
                                )
                                .width(Length::Fixed(COL_QTY_WIDTH)),
                                container(
                                    text(format!("£{:.2}", m.unit_cost))
                                        .size(14)
                                        .style(iced::theme::Text::Color(soft_ivory()))
                                )
                                .width(Length::Fixed(COL_UNIT_WIDTH)),
                                container(
                                    text(format!("£{:.2}", m.total_cost))
                                        .size(14)
                                        .style(iced::theme::Text::Color(soft_ivory()))
                                )
                                .width(Length::Fixed(COL_TOTAL_WIDTH)),
                                container(
                                    text(format!("{} days", m.lead_time_days))
                                        .size(14)
                                        .style(iced::theme::Text::Color(soft_ivory()))
                                )
                                .width(Length::Fixed(COL_LEAD_WIDTH)),
                                container(
                                    text(m.min_quantity.to_string())
                                        .size(14)
                                        .style(iced::theme::Text::Color(soft_ivory()))
                                )
                                .width(Length::Fixed(COL_MIN_WIDTH)),
                            ]
                            .spacing(8),
                        )
                    },
                );

                column![
                    text("Materials & costs")
                        .size(18)
                        .style(iced::theme::Text::Color(slate_blue())),
                    rows,
                ]
                .spacing(12)
                .into()
            }
            // Settings tab: configuration status for the estimating profile and rules.
            TabKind::Settings => {
                let settings = self.backend_state.settings();
                let status_text = if settings.configured {
                    "Profile: configured"
                } else {
                    "Profile: not configured yet"
                };

                column![
                    text("Estimate settings")
                        .size(18)
                        .style(iced::theme::Text::Color(slate_blue())),
                    text(status_text)
                        .size(16)
                        .style(iced::theme::Text::Color(soft_ivory())),
                    text(&settings.description)
                        .size(14)
                        .style(iced::theme::Text::Color(cool_gray())),
                ]
                .spacing(10)
                .into()
            }
            // Planning tab: indicative ordering timeline derived from lead times.
            TabKind::Advanced => {
                let materials = demo_materials();
                let rows = materials
                    .iter()
                    .fold(column![], |col, m| {
                        col.push(
                            column![
                                text(m.name)
                                    .size(14)
                                    .style(iced::theme::Text::Color(soft_ivory())),
                                text(format!(
                                    "Order at least {} units approximately {} days before installation.",
                                    m.min_quantity, m.lead_time_days
                                ))
                                .size(13)
                                .style(iced::theme::Text::Color(cool_gray())),
                            ]
                            .spacing(2),
                        )
                    })
                    .spacing(8);

                column![
                    text("Order planning")
                        .size(18)
                        .style(iced::theme::Text::Color(slate_blue())),
                    text("Indicative order timings based on current lead times.")
                        .size(14)
                        .style(iced::theme::Text::Color(cool_gray())),
                    rows,
                ]
                .spacing(12)
                .into()
            }
        };

        let card_inner = column![header, content]
            .spacing(16)
            .max_width(900)
            .align_items(Alignment::Start);

        // Fixed-size card so that switching tabs keeps the overall layout stable.
        // Any overflow is handled by the scrollable container above.
        let card = container(card_inner)
            .padding(24)
            .width(Length::Fixed(900.0))
            .height(Length::Fixed(600.0))
            .style(iced::theme::Container::Custom(Box::new(
                |_t: &iced::Theme| {
                    use iced::widget::container;
                    container::Appearance {
                        background: Some(charcoal().into()),
                        border: iced::Border {
                            radius: 8.0.into(),
                            width: 0.0,
                            color: Color::BLACK,
                        },
                        ..Default::default()
                    }
                },
            )));

        // Make the content scroll independently so the tab strip remains fixed at the top.
        let scroll = scrollable(card).height(Length::Fill);

        let root = column![tabs_row, scroll]
            .spacing(12)
            .padding([16, 24])
            .align_items(Alignment::Center);

        container(root)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(
                |_t: &iced::Theme| {
                    use iced::widget::container;
                    container::Appearance {
                        background: Some(slate_blue().into()),
                        ..Default::default()
                    }
                },
            )))
            .into()
    }
}

fn tab_button<'a>(label: &str, tab: TabKind, active_tab: TabKind) -> button::Button<'a, Message> {
    let is_active = tab == active_tab;
    let color = if is_active { soft_ivory() } else { cool_gray() };
    let accent = if is_active { slate_blue() } else { charcoal() };

    let label = text(label).size(16).style(iced::theme::Text::Color(color));

    let button_content = column![
        label,
        // Simple accent line under the active tab label.
        container(row![])
            .width(Length::Fixed(40.0))
            .height(Length::Fixed(2.0))
            .style(iced::theme::Container::Custom(Box::new(
                move |_t: &iced::Theme| {
                    use iced::widget::container;
                    container::Appearance {
                        background: if is_active { Some(accent.into()) } else { None },
                        ..Default::default()
                    }
                }
            ))),
    ]
    .spacing(4)
    .align_items(Alignment::Center);

    button(button_content)
        .on_press(Message::TabSelected(tab))
        .style(iced::theme::Button::Text)
}
