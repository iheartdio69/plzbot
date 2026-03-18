use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct TradeRequest {
    #[serde(rename = "tokenMint")]
    token_mint: String,
    side: String,
    #[serde(rename = "uiInputAmount")]
    ui_input_amount: String,
    #[serde(rename = "slippageBasisPoint")]
    slippage_basis_point: u32,
    #[serde(rename = "confirmationMode")]
    confirmation_mode: String,
}

#[derive(Debug, Deserialize)]
struct TradeResponse {
    status: bool,
    data: Option<TradeData>,
}

#[derive(Debug, Deserialize)]
struct TradeData {
    signature: Option<String>,
    status: Option<String>,
}

pub async fn buy(
    api_key: &str,
    token_mint: &str,
    sol_amount: f64,
) -> Result<String> {
    if api_key.trim().is_empty() {
        return Err(anyhow::anyhow!("FRONTRUN_API_KEY not set"));
    }

    let client = Client::new();
    let req = TradeRequest {
        token_mint: token_mint.to_string(),
        side: "BUY".to_string(),
        ui_input_amount: format!("{:.4}", sol_amount),
        slippage_basis_point: 1000,
        confirmation_mode: "ASYNC".to_string(),
    };

    let resp = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        client
            .post("https://solana.frontrun.pro/api/v1/trading-api/trade")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&req)
            .send()
    ).await??;

    let trade: TradeResponse = resp.json().await?;

    if trade.status {
        let sig = trade.data
            .and_then(|d| d.signature)
            .unwrap_or_else(|| "unknown".to_string());
        Ok(sig)
    } else {
        Err(anyhow::anyhow!("Trade failed"))
    }
}

pub async fn sell_percent(
    api_key: &str,
    token_mint: &str,
    percent: f64,
) -> Result<String> {
    if api_key.trim().is_empty() {
        return Err(anyhow::anyhow!("FRONTRUN_API_KEY not set"));
    }

    let client = Client::new();
    let body = serde_json::json!({
        "tokenMint": token_mint,
        "side": "SELL",
        "sellPercent": percent,
        "slippageBasisPoint": 1000,
        "confirmationMode": "ASYNC"
    });

    let resp = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        client
            .post("https://solana.frontrun.pro/api/v1/trading-api/trade")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
    ).await??;

    let trade: TradeResponse = resp.json().await?;

    if trade.status {
        let sig = trade.data
            .and_then(|d| d.signature)
            .unwrap_or_else(|| "unknown".to_string());
        Ok(sig)
    } else {
        Err(anyhow::anyhow!("Sell failed"))
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub mint: String,
    pub entry_fdv: f64,
    pub sol_spent: f64,
    pub opened_ts: i64,
    pub tp1_hit: bool,
    pub tp2_hit: bool,
    pub closed: bool,
}

impl Position {
    pub fn new(mint: &str, entry_fdv: f64, sol_spent: f64, now_ts: i64) -> Self {
        Self {
            mint: mint.to_string(),
            entry_fdv,
            sol_spent,
            opened_ts: now_ts,
            tp1_hit: false,
            tp2_hit: false,
            closed: false,
        }
    }

    pub fn check_exits(
        &mut self,
        current_fdv: f64,
        tp1_mult: f64,
        tp2_mult: f64,
        sl_pct: f64,
    ) -> Option<ExitSignal> {
        if self.closed {
            return None;
        }

        let mult = current_fdv / self.entry_fdv;

        // Stop loss
        if mult <= (1.0 - sl_pct) {
            self.closed = true;
            return Some(ExitSignal::StopLoss);
        }

        // TP2
        if !self.tp2_hit && mult >= tp2_mult {
            self.tp2_hit = true;
            return Some(ExitSignal::TakeProfit2);
        }

        // TP1
        if !self.tp1_hit && mult >= tp1_mult {
            self.tp1_hit = true;
            return Some(ExitSignal::TakeProfit1);
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExitSignal {
    TakeProfit1,  // sell 50%
    TakeProfit2,  // sell 25%
    StopLoss,     // sell 100%
}
