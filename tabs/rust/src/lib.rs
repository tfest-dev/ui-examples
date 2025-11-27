use serde::{Deserialize, Serialize};
use std::path::Path;

/// High-level tabs in the example application.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TabKind {
    Overview,
    Logs,
    Settings,
    Advanced,
}

/// Shared application state owned by the backend.
///
/// In a production system this would hold live data fetched from services
/// or derived from user actions. Here it uses a concise, static model
/// to illustrate the pattern while keeping the API realistic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub overview: OverviewSummary,
    pub bom: Vec<BomItem>,
    pub settings: SettingsSummary,
    pub advanced: AdvancedSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewSummary {
    pub title: String,
    pub status: String,
    pub key_metrics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomItem {
    pub name: String,
    pub quantity: u32,
    pub unit_cost: f32,
    pub total_cost: f32,
    pub lead_time_days: u32,
    pub min_quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsSummary {
    pub configured: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSummary {
    pub notes: String,
}

impl AppState {
    /// Construct a sample application state suitable for local runs and initial integration.
    ///
    /// `bom_path` is expected to point to a CSV file with columns:
    /// name, quantity, unit_cost, total_cost, lead_time_days, min_quantity.
    pub fn demo_with_bom_path<P: AsRef<Path>>(bom_path: P) -> Self {
        let overview = OverviewSummary {
            title: "Service overview".to_string(),
            status: "All systems nominal".to_string(),
            key_metrics: vec![
                "Latency: 120ms avg".to_string(),
                "Error rate: 0.2%".to_string(),
                "Active users: 1,245".to_string(),
            ],
        };

        let settings = SettingsSummary {
            configured: true,
            description: "Core credentials and thresholds are configured. Details are kept in the backend layer.".to_string(),
        };

        let advanced = AdvancedSummary {
            notes:
                "Space for diagnostic tools, import/export utilities, or one-off power features."
                    .to_string(),
        };

        let bom = load_bom_from_csv(bom_path).unwrap_or_else(|_| demo_bom());

        Self {
            overview,
            bom,
            settings,
            advanced,
        }
    }

    pub fn overview(&self) -> &OverviewSummary {
        &self.overview
    }

    pub fn bom(&self) -> &[BomItem] {
        &self.bom
    }

    pub fn settings(&self) -> &SettingsSummary {
        &self.settings
    }

    pub fn advanced(&self) -> &AdvancedSummary {
        &self.advanced
    }
}

fn load_bom_from_csv<P: AsRef<Path>>(path: P) -> Result<Vec<BomItem>, csv::Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let mut items = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if record.len() < 6 {
            continue;
        }
        let name = record[0].to_string();
        let quantity = record[1].parse().unwrap_or(0);
        let unit_cost = record[2].parse().unwrap_or(0.0);
        let total_cost = record[3].parse().unwrap_or(0.0);
        let lead_time_days = record[4].parse().unwrap_or(0);
        let min_quantity = record[5].parse().unwrap_or(0);

        items.push(BomItem {
            name,
            quantity,
            unit_cost,
            total_cost,
            lead_time_days,
            min_quantity,
        });
    }

    Ok(items)
}

fn demo_bom() -> Vec<BomItem> {
    vec![
        BomItem {
            name: "Steel frame sections".to_string(),
            quantity: 120,
            unit_cost: 45.50,
            total_cost: 5460.0,
            lead_time_days: 21,
            min_quantity: 50,
        },
        BomItem {
            name: "Electrical fixtures".to_string(),
            quantity: 80,
            unit_cost: 32.00,
            total_cost: 2560.0,
            lead_time_days: 14,
            min_quantity: 40,
        },
        BomItem {
            name: "Finishing materials".to_string(),
            quantity: 200,
            unit_cost: 12.75,
            total_cost: 2550.0,
            lead_time_days: 10,
            min_quantity: 100,
        },
    ]
}
