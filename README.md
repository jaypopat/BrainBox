# Brainbox - Second Brain

Brainbox is a personal knowledge management application inspired by Obsidian, designed to help you organize and link your thoughts using Markdown. The app supports basic note-taking functionality, Markdown editing, and internal note linking.

## Features

- **Markdown Support:** Write notes using Markdown syntax.
- **Internal Linking:** Link notes together using `[[Note Title]]` syntax.
- **Note Management:** Create, edit, and delete notes easily.
- **Live Markdown Preview:** See your Markdown content rendered in real-time.
- **Search Functionality:** Quickly find notes by title or content.

## Project Structure

The codebase is organized in a modular fashion to separate concerns and improve maintainability:


- `main.rs`: Sets up and runs the application using `eframe`.
- `app.rs`: Contains the main `AppState` struct, which manages the application state, including a collection of notes and the current active note.
- `node.rs`: Defines the `Node` struct representing a note, including attributes such as title, content, and links to other notes.
- `ui.rs`: Handles the user interface rendering and interactions using `egui`.

## Getting Started

### Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust installation)

### Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/jaypopat/BrainBox.git
   cd BrainBox

2. **Run the application**:

   ```bash
   cargo run
   ```
