use clap::Parser;
use chrono::Local;
use reqwest::Client;
use std::time::Instant;
use tokio::time::{sleep, Duration};

#[derive(Parser, Debug, PartialEq)]
#[command(name = "din")]
#[command(about = "CLI for making periodic HTTP requests.")]
struct Args {
    #[arg(value_name = "URL")]
    url: String,

    #[arg(
        short,
        long,
        default_value_t = 120,
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    interval: u64,

    #[arg(
        short,
        long,
        default_value_t = 10,
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    timeout: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let url = args.url;

    println!("🚀 Starting monitoring of: {}", url);
    println!("⏱️  Interval: {} seconds", args.interval);
    println!("⏳ Timeout: {} seconds", args.timeout);
    println!("Press Ctrl+C to stop\n");

    let client = Client::builder()
        .timeout(Duration::from_secs(args.timeout))
        .build()
        .unwrap();

    let mut request_count = 0;
    let mut success_count = 0;
    let mut error_count = 0;

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("\n🛑 Signal received. Closing immediately...");
                break;
            }

            _ = async {
                request_count += 1;
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

                let start = Instant::now();
                let result = client.get(&url).send().await;
                let elapsed = start.elapsed().as_secs_f64();

                match result {
                    Ok(response) => {
                        let status = response.status();
                        if status.is_success() {
                            success_count += 1;
                            println!(
                                "✅ [{}] #{} OK - {:.2}s",
                                timestamp, request_count, elapsed
                            );
                        } else {
                            error_count += 1;
                            println!(
                                "⚠️  [{}] #{} HTTP {} - {:.2}s",
                                timestamp, request_count, status, elapsed
                            );
                        }
                    }
                    Err(error) => {
                        error_count += 1;

                        println!(
                            "❌ [{}] #{} ERROR - {} - {:.2}s",
                            timestamp,
                            request_count,
                            error,
                            elapsed
                        );
                    }
                }

                sleep(Duration::from_secs(args.interval)).await;
            } => {}
        }
    }

    println!("\n📊 Final statistics:");
    println!("   Total: {}", request_count);

    if request_count > 0 {
        println!(
            "   Success: {} ({:.1}%)",
            success_count,
            success_count as f64 / request_count as f64 * 100.0
        );
        println!(
            "   Errors: {} ({:.1}%)",
            error_count,
            error_count as f64 / request_count as f64 * 100.0
        );
    }

    println!("👋 END.");
}


/* 
| TESTS
|
*/
#[cfg(test)]
mod tests {
    use super::*;

    fn parse_args(args: &[&str]) -> Result<Args, clap::error::Error> {
        Args::try_parse_from(args)
    }

    #[test]
    fn accepts_url() {
        let args = parse_args(&["din", "https://example.com"]).unwrap();

        assert_eq!(args.url, "https://example.com");
    }

    #[test]
    fn uses_default_values() {
        let args = parse_args(&["din", "https://example.com"]).unwrap();

        assert_eq!(args.interval, 120);
        assert_eq!(args.timeout, 10);
    }

    #[test]
    fn accepts_custom_interval() {
        let args = parse_args(&[
            "din",
            "https://example.com",
            "--interval",
            "60",
        ])
        .unwrap();

        assert_eq!(args.interval, 60);
    }

    #[test]
    fn accepts_custom_timeout() {
        let args = parse_args(&[
            "din",
            "https://example.com",
            "--timeout",
            "5",
        ])
        .unwrap();

        assert_eq!(args.timeout, 5);
    }

    #[test]
    fn rejects_missing_url() {
        let result = parse_args(&["din"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_interval() {
        let result = parse_args(&[
            "din",
            "https://example.com",
            "--interval",
            "0",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_timeout() {
        let result = parse_args(&[
            "din",
            "https://example.com",
            "--timeout",
            "0",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_numeric_interval() {
        let result = parse_args(&[
            "din",
            "https://example.com",
            "--interval",
            "abc",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_numeric_timeout() {
        let result = parse_args(&[
            "din",
            "https://example.com",
            "--timeout",
            "abc",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_unknown_option() {
        let result = parse_args(&[
            "din",
            "https://example.com",
            "--unknown",
        ]);

        assert!(result.is_err());
    }
    
}