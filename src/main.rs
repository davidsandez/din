use clap::Parser;
use chrono::Local;
use reqwest::Client;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Instant;
use tokio::time::{sleep, Duration};

#[derive(Parser, Debug)]
#[command(name = "din")]
#[command(about = "CLI para realizar peticiones HTTP periódicas.")]
struct Args {
    /// URL a monitorear (posicional)
    url: Option<String>,

    /// URL alternativa usando flag
    #[arg(short, long)]
    u: Option<String>,

    /// Intervalo en segundos entre peticiones
    #[arg(short, long, default_value_t = 120)]
    interval: u64,

    /// Timeout en segundos
    #[arg(short, long, default_value_t = 10)]
    timeout: u64,

    /// Modo verbose
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let url = match args.url.or(args.u) {
        Some(u) => u,
        None => {
            eprintln!("Debe especificar una URL (din https://example.com o -u URL)");
            std::process::exit(1);
        }
    };

    println!("🚀 Iniciando monitoreo de: {}", url);
    println!("⏱️  Intervalo: {} segundos", args.interval);
    println!("⏳ Timeout: {} segundos", args.timeout);
    println!("{}", "=".repeat(60));
    println!("Presiona Ctrl+C para detener el monitoreo\n");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error configurando handler de Ctrl+C");

    let client = Client::builder()
        .timeout(Duration::from_secs(args.timeout))
        .build()
        .unwrap();

    let mut request_count = 0;
    let mut success_count = 0;
    let mut error_count = 0;

    while running.load(Ordering::SeqCst) {
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
                        "✅ [{}] Petición #{}: OK - Tiempo: {:.2}s",
                        timestamp, request_count, elapsed
                    );
                } else {
                    error_count += 1;
                    println!(
                        "⚠️  [{}] Petición #{}: HTTP {} - Tiempo: {:.2}s",
                        timestamp, request_count, status, elapsed
                    );
                }

                if args.verbose {
                    println!("   └─ Status Code: {}", status);
                    println!(
                        "   └─ Content-Type: {:?}",
                        response.headers().get("content-type")
                    );
                }
            }
            Err(e) => {
                error_count += 1;
                if e.is_timeout() {
                    println!(
                        "❌ [{}] Petición #{}: TIMEOUT (>{}s)",
                        timestamp, request_count, args.timeout
                    );
                } else {
                    println!(
                        "❌ [{}] Petición #{}: ERROR DE CONEXIÓN",
                        timestamp, request_count
                    );
                }

                if args.verbose {
                    println!("   └─ Detalles: {}", e);
                }
            }
        }

        if args.verbose {
            println!("   └─ Esperando {} segundos...\n", args.interval);
        }

        sleep(Duration::from_secs(args.interval)).await;
    }

    println!("\n\n{}", "=".repeat(60));
    println!("🛑 Monitoreo detenido por el usuario");
    println!("{}", "=".repeat(60));
    println!("📊 Estadísticas:");
    println!("   • Total de peticiones: {}", request_count);

    if request_count > 0 {
        println!(
            "   • Exitosas: {} ({:.1}%)",
            success_count,
            success_count as f64 / request_count as f64 * 100.0
        );
        println!(
            "   • Con errores: {} ({:.1}%)",
            error_count,
            error_count as f64 / request_count as f64 * 100.0
        );
    }

    println!("{}", "=".repeat(60));
    println!("👋 ¡Hasta luego!");
}