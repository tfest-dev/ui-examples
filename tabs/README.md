# Tabs-based Rust GUI Example

## Overview

This example shows a **multi-tab** desktop application backed by Rust.

It is intended for tools that bring several related workflows together in one place, such as:

- Operator or admin consoles
- Dashboards with multiple views of the same data
- Internal tools with distinct sections for overview, logs, and configuration

The key idea is that each tab focuses on one area of functionality, while a single Rust backend keeps the overall state consistent.

<img src=screenshots/quote-tab-screenshot.png alt="Tabs UI - Quote page" width="400" height="300"/>
<img src=screenshots/breakdown-tab-screenshot.png alt="Tabs UI - Breakdown page" width="400" height="300"/>
<img src=screenshots/planning-tab-screenshot.png alt="Tabs UI - Planning page" width="400" height="300"/>
<img src=screenshots/settings-tab-screenshot.png alt="Tabs UI - Settings page" width="400" height="300"/>

## What users see

- A row of tabs across the top (for example: Overview, Logs, Settings, Advanced).
- A main content area that changes depending on the selected tab.
- Clear, labelled sections rather than multiple floating windows.

This makes it easy to explain “where things live” in the app and gives users a predictable way to move between views.

## Technology at a glance

- Desktop application built with a Rust backend and a Rust-native UI using **Iced** (no browser required).
- The backend crate under `rust/` owns shared state and domain data.
- The Iced UI under `ui/` renders the tabs and calls into the backend to populate each view.

## How it behaves

When someone uses the app:

1. They pick a tab that matches their current task (e.g. "Overview" or "Settings").
2. The UI asks the backend for the relevant data for that section.
3. The backend returns structured information (summaries, logs, configuration status).
4. The visible panel updates while the rest of the state is kept consistent behind the scenes.

From a client perspective, this allows separate teams (operations, compliance, engineering) to get tailored views over the same underlying system.

## What can be customized

Within the tabs pattern, several aspects are intentionally flexible per project:

- **Tab names and count**: which sections exist (Overview, Logs, Settings, Advanced, or custom ones).
- **Content per tab**: metrics and charts in Overview, which log streams or filters appear in Logs, what controls are exposed in Settings, and which power tools live under Advanced.
- **Visual design**: colours, typography, spacing, and branding to match an existing product or design system.
- **Navigation behaviour**: whether tabs are always visible, how deep links / default tabs work, and what happens when there are validation issues in one section.

The overall structure (a single window with multiple tabs backed by one Rust state) stays the same so that behaviour remains predictable.

## Security and data handling

- Sensitive configuration (API keys, tokens) is stored only in the Rust backend.
- The Settings tab can show high-level status (e.g. "configured" vs "not configured") without exposing raw secrets.
- Validation, rate limiting, and audit-friendly behaviour can all be centralised in the Rust layer.
