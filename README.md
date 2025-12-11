# TD01 – Rust Async Workshop with Tokio  
# TD02 – Rust WebSocket Workshop with Tokio  
### by Jennifer El Achkar ESILV A4 (FinTech)

## Overview
This repository contains two full Rust workshops built using Tokio. They cover:

- **TD1 — Asynchronous Programming & API Aggregation**
  - Async tasks with Tokio
  - Calling multiple APIs concurrently (Alpha Vantage, Finnhub, mock API)
  - Safe error handling (system stays alive if one API fails)
  - Database persistence (PostgreSQL)
  - Graceful shutdown with Ctrl + C
  - Structured and timestamped logs

- **TD2 — Real-Time WebSocket Dashboard**
  - WebSocket server built with Tokio
  - Real-time broadcasting
  - Interactive dashboard (HTML + JS)
  - Stable async architecture

## Features
 Fully asynchronous architecture (Tokio runtime)  
 Multiple API sources aggregated concurrently  
 Automatic retries + error resilience  
 PostgreSQL storage with SQL schema  
 Configurable through `.env`  
 WebSocket server for live message broadcasting  
 Clean project structure and logs  

## Project Structure
```
TD1_2_Rust_Tokio/
│
├── TD1/
│   ├── Cargo.toml
│   ├── rust-1.md
│   └── src/
│       └── main.rs
│
├── TD2/
│   ├── Cargo.toml
│   ├── rust-2.md
│   └── src/
│       └── main.rs
│
├── schema.sql
├── README.md
└── Cargo.toml
```

## How to Run the Project
1. **Clone the repository**
   ```bash
   git clone https://github.com/Jenniferachkar/TD1_2_Rust_Tokio
   cd TD1_2_Rust_Tokio
   cargo check
   cargo build
   ```

2. **Set up the PostgreSQL database**
   Create the database:
   ```bash
   createdb stockdb
   ```
   Import the schema:
   ```bash
   psql -U postgres -d stockdb -f schema.sql
   ```

3. **Create a `.env` file at the project root**
   ```ini
   ALPHA_VANTAGE_KEY=your_api_key
   FINNHUB_KEY=your_api_key
   DATABASE_URL=postgresql://user:password@localhost/stockdb
   ```

4. **TD1 — Run the Async Stock Aggregator**
   ```bash
   cd TD1
   cargo run
   ```
   Once launched, the program will:
   - Fetch stock prices every minute
   - Store the data into PostgreSQL
   - Display live structured logs
   - Continue running even if one API fails  
   Stop safely using Ctrl + C

5. **TD2 — Run the WebSocket Server**
   ```bash
   cd TD2
   cargo run
   ```
   Then open:
   ```
   dashboard.html
   ```
   This page will connect to the WebSocket server and display messages.

## How to Test the WebSocket
Create a file named `test.html`:
```html
<!DOCTYPE html>
<html>
<body>
<script>
const ws = new WebSocket("ws://127.0.0.1:8080");
ws.onopen = () => ws.send("Hello Nathan c'est Jenny");
ws.onmessage = (e) => console.log("Received:", e.data);
</script>
</body>
</html>
```
Open it in Chrome → F12 → Console. You should see:
```
Received: Hello Nathan c'est Jenny
```
