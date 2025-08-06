# BlockSavy

🚀 A high-performance, AI-powered crypto trend analyzer built with Rust.

BlockSavy tracks historical cryptocurrency data, detects technical indicators, and provides buy/sell insights using AI-powered logic.

## 🛠 Features

- Fetch historical crypto prices via CoinGecko API
- Analyze trends using RSI and other indicators
- AI-generated trade opinions (Buy, Sell, Hold)
- RESTful API built with Axum (Rust)
- Future: Frontend dashboard, backtesting, Telegram alerts

## 📦 Tech Stack

- **Backend**: Rust + Axum
- **Crypto Data**: CoinGecko API
- **Technical Analysis**: Custom RSI logic (expandable)
- **AI Layer**: Rule-based (LLM integration optional)
- **Frontend**: (Coming soon with Yew or React)

## 🚀 Getting Started

1. Clone the repository:
    ```bash
    git clone https://github.com/diers68-source/blocksavy.git
    cd blocksavy
    ```

2. Install Rust: [https://rustup.rs](https://rustup.rs)

3. Run the backend server:
    ```bash
    cargo run
    ```

4. Access the API:
    - `/` → Health check
    - `/prices` → 7-day BTC prices
    - `/signals` → RSI-based trading signal

## 📈 Roadmap

- [ ] Add support for multiple coins
- [ ] Enhance signal engine with MACD, Bollinger Bands
- [ ] Integrate AI-generated explanations
- [ ] Add real-time alerts and frontend dashboard

---

**Created by @diers68** • MIT License
