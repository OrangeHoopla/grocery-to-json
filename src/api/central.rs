use axum::{
    extract::{Multipart, Path, Request},
    http::{header, HeaderMap},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use uuid::Uuid;

use crate::{dao::{image_dao::Image, raw_text_dao::RawText}, image_processors::tesseract_processor};

// use crate::{ocrengines, text_processors::{self, wholefoods_processor::GroceryList}};

pub fn get_routes() -> Router {
    Router::new()
        .route("/json", get(json))
        .route("/upload", post(upload))
        .route("/", get(handler))
    // .route("/{id}", get(parse_file))
        .route("/submit/{id}", post(submit_entry))
    // .route("/all", get(getall))
}

async fn handler() -> Html<&'static str> {
    // let html_content =
    // read_html_from_file("index.html").await.unwrap_or_else(|_| {
    //     "<h1>Error loading HTML file</h1>".to_string()
    // });
    Html(
        r#"
        <!doctype html>
        <html>
            <head>
                <title>Upload something!</title>
            </head>
            <body>
                <form action="/api/upload" method="post" enctype="multipart/form-data">
                    <div>
                        <label>
                            Upload file:
                            <input type="file" name="file" multiple>
                        </label>
                    </div>

                    <div>
                        <input type="submit" value="Upload files">
                    </div>
                </form>
            </body>
        </html>
        "#,
    )
}

/*
Test
 */
async fn json(req: Request) -> Json<Value> {
    let (parts, _body) = req.into_parts();
    println!(" {:#?} ", parts.method);
    println!(" {:#?} ", parts.version);
    println!(" {:#?} ", parts.uri);
    println!(" {:#?} ", parts.headers);
    Json(json!({ "data": 42 }))
}

/*
Takes Uploaded file and stores it locally
returning the id it was saved as to be called upon later
 */
async fn upload(_headers: HeaderMap, mut multipart: Multipart) -> impl IntoResponse {
    let id = Uuid::new_v4();
    // let a = headers.get("store").unwrap().to_str().unwrap();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let _name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        fs::write(format!("images/{}", id), data).expect("Error writing Image to File");
    }

    let test = Image {
        file_name: id.to_string(),
        created: Utc::now(),
    };
    Image::save(test).await;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
    (headers, id.to_string())
}

// async fn _parse_file_raw(Path(id): Path<String>) -> impl IntoResponse {
//     let result = ocrengines::tesseract_engine::result(id);

//     let mut headers = HeaderMap::new();
//     headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
//     (headers, result)

// }

/*
Takes in id of uploaded file for it to be proccessed
 */
// async fn parse_file(Path(id): Path<String>) -> Json<Value> {
//     // let (_parts, _body) = req.into_parts();
//     let client_uri = "mongodb://mongo:27017";
//     let client = Client::with_uri_str(&client_uri).await;
//     let my_coll: Collection<Document> = client.unwrap()
//         .database("test")
//         .collection("Images");
//     let store = my_coll.find_one(doc! {
//         "id": id.to_owned()
//     })
//     .await
//     .unwrap()
//     .unwrap()
//     .get("store")
//     .unwrap().to_string();

//     println!("*********************************************************");
//     println!("{}",store.as_str());

//     let result = ocrengines::tesseract_engine::result(id.to_owned());

//     let reciept = match store.as_str() {
//             "\"WholeFoods\"" => text_processors::wholefoods_processor::parse(result),
//             "\"Aldi\"" => text_processors::aldi_processor::parse(result),
//             _ => {println!("FAILED");
//             text_processors::aldi_processor::parse(result)}
//         };

//     let wow: Value = serde_json::to_value(reciept).unwrap();
//     Json(json!(wow))

// }

/**
 * Takes in json for GroceryList item
 * and converts it to be submitted
 */
async  fn submit_entry(Path(id): Path<String>) -> Json<RawText> {

    let result = tesseract_processor::result(id);

    let gret = RawText {
        raw_text: result.clone(),
        created: Utc::now(),
        image_processor: "tesseract_processor".to_owned(),
    };

    let temp = RawText {
        raw_text: result.clone(),
        created: Utc::now(),
        image_processor: "tesseract_processor".to_owned(),
    };


    RawText::save(gret).await;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    Json(temp)
}

/*
returns the latest 5 entries into the grocery list
 */
// async fn getall() -> Json<Value> {
//     let client_uri = "mongodb://mongo:27017";
//     let client = Client::with_uri_str(&client_uri).await;
//     let my_coll: Collection<GroceryList> = client.unwrap()
//         .database("test")
//         .collection("Entries");
//     let cursor = my_coll.find(
//         doc! {}
//     );
//     let mut wow = cursor
//     .limit(5)
//     .sort(doc! {"entrydate": -1})
//     .await.unwrap();
//     let mut wwww: Vec<GroceryList> = vec![];

//     // while !wow.current().is_empty() {
//     //     wwww.push(wow.deserialize_current().unwrap());
//     //     println!("{:?}",wow.deserialize_current().unwrap());
//     //     wow.advance().await.unwrap_or_else(None);
//     // }

//     while  wow.advance().await.unwrap() {
//         wwww.push(wow.deserialize_current().unwrap());
//         // println!("{:?}",wow.deserialize_current().unwrap());
//     }

//     // println!("{:?}",wow);

//     Json(json!(wwww))

// }

#[allow(dead_code)]
enum Stores {
    WholeFoods,
    Aldi,
}
