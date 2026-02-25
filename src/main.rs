use clap::Parser;
use chrono::Local;
use reqwest::Client;
use std::time::Instant;
use tokio::time::{sleep, Duration};

#[derive(Parser, Debug)]
#[command(name = "din")]
#[command(about = "CLI para realizar peticiones HTTP periódicas.")]
struct Args {
    url: Option<String>,

    #[arg(short, long)]
    u: Option<String>,

    #[arg(short, long, default_value_t = 120)]
    interval: u64,

    #[arg(short, long, default_value_t = 10)]
    timeout: u64,

    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let url = match args.url.or(args.u) {
        Some(u) => u,
        None => {
            eprintln!("Debe especificar una URL");
            std::process::exit(1);
        }
    };

    println!("🚀 Iniciando monitoreo de: {}", url);
    println!("Presiona Ctrl+C para detener\n");

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
                println!("\n🛑 Señal recibida. Cerrando inmediatamente...");
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
                    Err(_) => {
                        error_count += 1;
                        println!(
                            "❌ [{}] #{} ERROR/TIMEOUT",
                            timestamp, request_count
                        );
                    }
                }

                sleep(Duration::from_secs(args.interval)).await;
            } => {}
        }
    }

    println!("\n📊 Estadísticas finales:");
    println!("   Total: {}", request_count);

    if request_count > 0 {
        println!(
            "   Exitosas: {} ({:.1}%)",
            success_count,
            success_count as f64 / request_count as f64 * 100.0
        );
        println!(
            "   Errores: {} ({:.1}%)",
            error_count,
            error_count as f64 / request_count as f64 * 100.0
        );
    }

    println!("👋 Fin.");
}