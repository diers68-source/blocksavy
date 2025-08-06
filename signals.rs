use serde_json::json;

pub async fn analyze() -> serde_json::Value {
    let rsi = 28.0;
    let signal = if rsi < 30.0 {
        "Buy"
    } else if rsi > 70.0 {
        "Sell"
    } else {
        "Hold"
    };

    json!({
        "rsi": rsi,
        "signal": signal,
        "comment": "RSI suggests this may be a buying opportunity"
    })
}
