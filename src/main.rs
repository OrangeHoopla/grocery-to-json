use axum::{extract::DefaultBodyLimit, Router};
use clap::Parser;
use grocery_to_json::api;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = false)]
    server: bool,

    /// Number of times to greet
    #[arg(short, long)]
    file: String,
}

const PORT: &str = "8000";
const ADDRESS: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    if args.server {
        let app = Router::new()
            .nest_service("/api", api::central::get_routes())
            .layer(DefaultBodyLimit::max(1000000000))
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http());

        let listener = tokio::net::TcpListener::bind(format!("{}:{}", ADDRESS, PORT))
            .await
            .unwrap();

        println!("Server Starting");
        axum::serve(listener, app).await.unwrap();
    } else {
        println!("{}", args.file)
    }
}
