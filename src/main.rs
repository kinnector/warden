use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "wardend")]
#[command(about = "Kinnector Warden: Server EDR Daemon", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "/var/run/kinnector/telemetry.sock", help = "Path to core telemetry UDS socket")]
    telemetry_socket: String,

    #[arg(short, long, default_value = "/var/www/html", help = "Web application root directory for FIM and OSV scans")]
    web_root: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging with env-filter
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into()))
        .init();

    let args = Args::parse();
    
    let config = warden::WardenConfig {
        telemetry_socket: args.telemetry_socket,
        web_root: args.web_root,
    };
    
    warden::run_warden(config).await
}
