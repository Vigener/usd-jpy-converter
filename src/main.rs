use clap::Parser;
use chrono::Local;
use serde::Deserialize;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(name = "cmd")]
#[command(about = "USD↔JPY為替レート変換ツール", long_about = None)]
struct Args {
    /// ドルを円に変換
    #[arg(short = 'd', long = "dollar", value_name = "AMOUNT", conflicts_with = "yen")]
    dollar: Option<f64>,

    /// 円をドルに変換
    #[arg(short = 'y', long = "yen", value_name = "AMOUNT", conflicts_with = "dollar")]
    yen: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct ExchangeRateResponse {
    conversion_rates: ConversionRates,
}

#[derive(Deserialize, Debug)]
struct ConversionRates {
    #[serde(rename = "JPY")]
    jpy: f64,
}

fn get_exchange_rate() -> Result<f64, Box<dyn Error>> {
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
    let data: ExchangeRateResponse = response.json()?;
    Ok(data.conversion_rates.jpy)
}

fn main() {
    let args = Args::parse();

    // 少なくとも1つの引数が必要
    if args.dollar.is_none() && args.yen.is_none() {
        eprintln!("エラー: -d または -y オプションで金額を指定してください");
        eprintln!("使用例:");
        eprintln!("  cmd -d 100    # 100ドルを円に変換");
        eprintln!("  cmd -y 10000  # 10000円をドルに変換");
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

    if let Some(dollar_amount) = args.dollar {
        let yen_result = dollar_amount * rate;
        println!("💵 {} USD → 💴 {:.2} JPY", dollar_amount, yen_result);
    } else if let Some(yen_amount) = args.yen {
        let dollar_result = yen_amount / rate;
        println!("💴 {} JPY → 💵 {:.2} USD", yen_amount, dollar_result);
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
