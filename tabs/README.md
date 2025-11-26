# Tabs-based Rust GUI Example

## Overview

This example shows a **multi-tab** desktop application backed by Rust.

It is intended for tools that bring several related workflows together in one place, such as:

- Operator or admin consoles
- Dashboards with multiple views of the same data
- Internal tools with distinct sections for overview, logs, and configuration

The key idea is that each tab focuses on one area of functionality, while a single Rust backend keeps the overall state consistent.
