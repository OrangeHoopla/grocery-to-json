use axum::{extract::DefaultBodyLimit, Router};
use bson::doc;
use std::env;
use clap::Parser;
use grocery_to_json::{api, dao::mongo};
use tower_http::{cors::CorsLayer, trace::TraceLayer};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// if present become a service
    #[arg(short, long, default_value_t = false)]
    server: bool,

    /// file to parse locally
    #[arg(short, long, default_value_t = ("image.png".to_string()))]
    file: String,
}

const PORT: &str = "8000";
const ADDRESS: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    // run as server
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
    } 
    
    // TODO run against single file
    else {
        println!("{}", args.file);
        let test = mongo::MongoConnection::get_collection("Entries");
        println!("{}", args.file);
        let wow = test.client().await.unwrap().find_one(doc! {}).await.unwrap();
        println!("{}", args.file);
        
        println!("{:?}", wow);
        
    }
}
