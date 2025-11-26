# Graphical User Interface Examples

This directory contains example graphical user interfaces (GUIs) and "GUI wrappers" that I have produced.

Most examples use Rust for the glue between the user interface and the underlying services or business logic.

## Why use Rust between front end and back end?

Rust is used as the middle layer between UI code (web, desktop, or native widgets) and back‑end systems (APIs, databases, Python services, local tools, LLMs, etc.) because it is:

1. **Explicit**
2. **Memory and thread safe**
3. **Quick and Lightweight**
4. **Security and isolation of secrets**  

In all examples, the GUI and wrapper are designed so that **no secrets are passed directly through the visual/UI layer**. The Rust boundary is responsible for securely handling sensitive data and communication with back‑end systems.

## Examples

### Single Window

A minimal desktop window layout focused on a single task or workflow. This pattern is ideal for:

- Small internal tools
- Focused data entry or review screens
- Simple dashboards or status monitors



