use axum::{extract::DefaultBodyLimit, Router};
use chrono::Utc;
use clap::Parser;
use grocery_to_json::{
    api,
    dao::{image_dao::Image},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

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
        print!("Manual run");
        let test = Image {
            file_name: Uuid::new_v4().to_string(),
            created: Utc::now(),
        };
        Image::save(test).await;

        // println!("{}", env::var("MONGODB_DB").unwrap().as_str());

        // let client = Client::with_uri_str(env::var("MONGODB_URI").unwrap().as_str())
        //     .await
        //     .unwrap();
        // let image_client: mongodb::Collection<Image> = client.database(env::var("MONGODB_DB").unwrap().as_str()).collection("images");
        // let res = image_client.insert_one(Image{ file_name: Uuid::new_v4().to_string(), created: Utc::now()}).await;

        // println!("{}", args.file);
        // let test = mongo::MongoConnection::get_collection("Entries");
        // println!("{}", args.file);
        // let wow = test
        //     .client()
        //     .await
        //     .unwrap()
        //     .find_one(doc! {})
        //     .await
        //     .unwrap();
        // println!("{}", args.file);

        // println!("{:?}", wow);
    }
}
