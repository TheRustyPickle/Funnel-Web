# Funnel

A work-in-progress platform for visualizing Discord analytics, built with Rust and [egui](https://github.com/emilk/egui). This repository contains the user interface component of the project. While designed for WebAssembly (Wasm) compatibility, this can also be compiled into a native application.

## Getting Started

### Prerequisites

- **[Trunk CLI](https://trunkrs.dev)**: Ensure you have Trunk installed to serve the project.
- **Rust Toolchain**: Install Rust via [rustup](https://rustup.rs) if not already set up.

### Building and Running

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/funnel-web.git
    cd funnel-web
    ```

2. To run in WebAssembly:

    ```sh
    trunk serve --release
    ```

   Open your browser and navigate to `http://localhost:8080` to view the app.

3. To build and run as a native application:

    ```sh
    cargo run --release
    ```

## Planned Features

- [x] **Overview**: Summarizes key metrics such as total messages, unique users, most active channels and users. Includes a chart tracking member movement (e.g., joins and leaves).
- [x] **User Table**: Displays all users, including total messages, word counts, and other details.
- [ ] **Channel Table**: Provides message statistics for each channel.
- [ ] **Message Chart**: Visualizes total and deleted messages. Allows adding individual users for detailed analysis over daily, hourly, weekly, and monthly intervals.
- [ ] **User Activity Chart**: Shows active user counts over different timeframes (daily, hourly, weekly, monthly).
- [ ] **Common Words Analysis**: Highlights the most common words or phrases used in messages.
- [ ] **Channel Filter**: Allow filtering all analytics by the selected channels

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

Funnel-Web is licensed under the [MIT License](LICENSE).
