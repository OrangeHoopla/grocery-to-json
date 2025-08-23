use axum::{extract::DefaultBodyLimit, Router};
use clap::Parser;
use grocery_to_json::api;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// if present become a service
    #[arg(short, long, default_value_t = false)]
    server: bool,

    /// file to parse locally
    #[arg(short, long, default_value_t = String::from("image.png"))]
    file: String,
}

const PORT: &str = "8000";
const ADDRESS: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    divan::main();
    let args: Args = Args::parse();

    // run as server
    
    if args.server {
        println!("Server Starting...");
        let app = Router::new()
            .nest_service("/api", api::central::get_routes())
            .layer(DefaultBodyLimit::max(1000000000))
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http());

        let listener = tokio::net::TcpListener::bind(format!("{}:{}", ADDRESS, PORT))
            .await
            .unwrap();

        axum::serve(listener, app).await.unwrap();

    }
    // TODO run against single file
    else {

        // needs to become just regular way to run in terminal
        //edit
        // let result = sobel_transform::process_frame("IMG_5722.jpg".to_string(),"result.jpeg".to_string(),1);

        // let _ = result.save("result.jpeg");
        // let fer = rusty_tesseract::Image::from_dynamic_image(&result).unwrap();
        // let default_args = rusty_tesseract::Args::default();
        // let output = rusty_tesseract::image_to_string(&fer, &default_args).unwrap();
        // fs::write("edit.txt", output).expect("Should be able to write to `/foo/tmp`");
        

        // //original
        // let mut decoder = ImageReader::open("IMG_5722.jpg").unwrap()
        // .with_guessed_format().unwrap().into_decoder().unwrap();
        // let orientation = decoder.orientation().unwrap();
        // let mut dynamic_image = DynamicImage::from_decoder(decoder).unwrap();
        // dynamic_image.apply_orientation(orientation);

        // let img = rusty_tesseract::Image::from_dynamic_image(&dynamic_image).unwrap();
        // let default_args = rusty_tesseract::Args::default();
        // let output = rusty_tesseract::image_to_string(&img, &default_args).unwrap();
        // fs::write("original.txt", output).expect("Should be able to write to `/foo/tmp`");


    }
}
