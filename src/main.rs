use chrono::Local;
use clap::Parser;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug)]
enum AmountInput {
    Single(f64),
    Range(f64, f64),
}

#[derive(Parser, Debug)]
#[command(name = "ujcon")]
#[command(about = "USD↔JPY為替レート変換ツール", long_about = None)]
struct Args {
    /// ドルを円に変換（単一値またはレンジ: 例: 100 または 100-200）
    #[arg(
        short = 'd',
        long = "dollar",
        visible_aliases = ["usd", "USD", "u"],
        value_name = "AMOUNT",
        conflicts_with = "yen"
    )]
    dollar: Option<String>,

    /// 円をドルに変換（単一値またはレンジ: 例: 10000 または 10000-20000）
    #[arg(
        short = 'y',
        long = "yen",
        visible_aliases = ["jpy", "JPY", "j"],
        value_name = "AMOUNT",
        conflicts_with = "dollar"
    )]
    yen: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ExchangeRateResponse {
    // exchangerate-api.com と open.er-api.com 用
    Standard { conversion_rates: ConversionRates },
    // 代替フォーマット用
    Rates { rates: ConversionRates },
}

#[derive(Deserialize, Debug)]
struct ConversionRates {
    #[serde(rename = "JPY")]
    jpy: f64,
}

fn get_exchange_rate() -> Result<f64, Box<dyn Error>> {
    // テスト用: MOCK_RATE環境変数が設定されている場合はそれを使用
    if let Ok(mock_rate) = std::env::var("MOCK_RATE") {
        if let Ok(rate) = mock_rate.parse::<f64>() {
            return Ok(rate);
        }
    }
    
    // 複数のAPIエンドポイントを試す
    let urls = vec![
        "https://api.exchangerate-api.com/v4/latest/USD",
        "https://open.er-api.com/v6/latest/USD",
    ];

    let mut last_error = None;

    for url in urls {
        match try_get_rate(url) {
            Ok(rate) => return Ok(rate),
            Err(e) => {
                last_error = Some(e);
                continue;
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "すべてのAPIエンドポイントが失敗しました".into()))
}

fn try_get_rate(url: &str) -> Result<f64, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;

    // HTTPステータスコードをチェック
    if !response.status().is_success() {
        return Err(format!("HTTPエラー: {}", response.status()).into());
    }

    let data: ExchangeRateResponse = response.json()?;

    // 両方のAPIフォーマットに対応
    let rate = match data {
        ExchangeRateResponse::Standard { conversion_rates } => conversion_rates.jpy,
        ExchangeRateResponse::Rates { rates } => rates.jpy,
    };

    Ok(rate)
}

fn parse_amount(input: &str) -> Result<AmountInput, String> {
    // ハイフンでレンジかどうか判定
    if input.contains('-') {
        // レンジ形式: "100-200" のようなフォーマット
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() != 2 {
            return Err("レンジ形式が正しくありません。例: 100-200".to_string());
        }
        
        let start = parts[0].trim().parse::<f64>()
            .map_err(|_| format!("開始値 '{}' を数値としてパースできません", parts[0]))?;
        let end = parts[1].trim().parse::<f64>()
            .map_err(|_| format!("終了値 '{}' を数値としてパースできません", parts[1]))?;
        
        if start >= end {
            return Err("開始値は終了値より小さい必要があります".to_string());
        }
        
        Ok(AmountInput::Range(start, end))
    } else {
        // 単一値
        let value = input.trim().parse::<f64>()
            .map_err(|_| format!("値 '{}' を数値としてパースできません", input))?;
        Ok(AmountInput::Single(value))
    }
}

fn main() {
    let args = Args::parse();

    // 少なくとも1つの引数が必要
    if args.dollar.is_none() && args.yen.is_none() {
        eprintln!("エラー: -d または -y オプションで金額を指定してください");
        eprintln!("使用例:");
        eprintln!("  ujcon -d 100    # 100ドルを円に変換");
        eprintln!("  ujcon -y 10000  # 10000円をドルに変換");
        std::process::exit(1);
    }

    // 為替レートを取得
    let rate = match get_exchange_rate() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("エラー: 為替レートの取得に失敗しました: {}", e);
            std::process::exit(1);
        }
    };

    let now = Local::now();
    let timestamp = now.format("%Y年%m月%d日 %H:%M:%S").to_string();

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 USD/JPY 為替レート変換");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💱 現在のレート: 1 USD = {:.2} JPY", rate);
    println!("🕐 取得時刻: {}", timestamp);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    if let Some(dollar_input) = args.dollar {
        match parse_amount(&dollar_input) {
            Ok(AmountInput::Single(dollar_amount)) => {
                let yen_result = dollar_amount * rate;
                println!("💵 {} USD → 💴 {:.2} JPY", dollar_amount, yen_result);
            }
            Ok(AmountInput::Range(start, end)) => {
                let yen_start = start * rate;
                let yen_end = end * rate;
                println!("💵 {} - {} USD → 💴 {:.2} - {:.2} JPY", start, end, yen_start, yen_end);
            }
            Err(e) => {
                eprintln!("エラー: {}", e);
                std::process::exit(1);
            }
        }
    } else if let Some(yen_input) = args.yen {
        match parse_amount(&yen_input) {
            Ok(AmountInput::Single(yen_amount)) => {
                let dollar_result = yen_amount / rate;
                println!("💴 {} JPY → 💵 {:.2} USD", yen_amount, dollar_result);
            }
            Ok(AmountInput::Range(start, end)) => {
                let dollar_start = start / rate;
                let dollar_end = end / rate;
                println!("💴 {} - {} JPY → 💵 {:.2} - {:.2} USD", start, end, dollar_start, dollar_end);
            }
            Err(e) => {
                eprintln!("エラー: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_value() {
        match parse_amount("100") {
            Ok(AmountInput::Single(val)) => assert_eq!(val, 100.0),
            _ => panic!("Expected Single value"),
        }
    }

    #[test]
    fn test_parse_single_value_with_decimal() {
        match parse_amount("100.50") {
            Ok(AmountInput::Single(val)) => assert_eq!(val, 100.50),
            _ => panic!("Expected Single value"),
        }
    }

    #[test]
    fn test_parse_range() {
        match parse_amount("100-200") {
            Ok(AmountInput::Range(start, end)) => {
                assert_eq!(start, 100.0);
                assert_eq!(end, 200.0);
            }
            _ => panic!("Expected Range value"),
        }
    }

    #[test]
    fn test_parse_range_with_spaces() {
        match parse_amount("100 - 200") {
            Ok(AmountInput::Range(start, end)) => {
                assert_eq!(start, 100.0);
                assert_eq!(end, 200.0);
            }
            _ => panic!("Expected Range value"),
        }
    }

    #[test]
    fn test_parse_range_decimal() {
        match parse_amount("599.5-699.99") {
            Ok(AmountInput::Range(start, end)) => {
                assert_eq!(start, 599.5);
                assert_eq!(end, 699.99);
            }
            _ => panic!("Expected Range value"),
        }
    }

    #[test]
    fn test_parse_invalid_range_start_greater() {
        match parse_amount("200-100") {
            Err(msg) => assert!(msg.contains("開始値は終了値より小さい")),
            _ => panic!("Expected error for invalid range"),
        }
    }

    #[test]
    fn test_parse_invalid_range_equal() {
        match parse_amount("100-100") {
            Err(msg) => assert!(msg.contains("開始値は終了値より小さい")),
            _ => panic!("Expected error for equal values"),
        }
    }

    #[test]
    fn test_parse_invalid_input() {
        match parse_amount("abc") {
            Err(msg) => assert!(msg.contains("数値としてパースできません")),
            _ => panic!("Expected error for invalid input"),
        }
    }

    #[test]
    fn test_parse_invalid_range_format() {
        match parse_amount("100-200-300") {
            Err(msg) => assert!(msg.contains("レンジ形式が正しくありません")),
            _ => panic!("Expected error for invalid range format"),
        }
    }
}
