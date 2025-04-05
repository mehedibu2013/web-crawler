![Rust](https://img.shields.io/badge/Rust-1.75.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)

# Web Crawler

A simple multithreaded web crawler written in Rust. This tool systematically browses the web starting from a seed URL, extracts links from HTML pages, and avoids revisiting already-crawled URLs.

## Features

- Crawls websites starting from a seed URL.
- Extracts all `<a>` tags with `href` attributes from HTML pages.
- Resolves relative URLs against the base URL.
- Avoids revisiting already-crawled URLs using a thread-safe `DashSet`.
- Supports crawling up to a specified depth limit.
- Built with concurrency using `tokio` for efficient performance.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)

## Prerequisites

Before running the web crawler, ensure you have the following installed:

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Cargo**: Rust's package manager (installed automatically with Rust).
- **Git**: Optional, for cloning the repository.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/username/web-crawler.git
   cd web-crawler
2. Build the project:
   ```bash
   cargo build
3. Run the  project:
   ```bash
   cargo run
   
