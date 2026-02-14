use actix_web::{web, App, HttpServer, Responder};
use firebase_rs::Firebase;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let firebase = Firebase::new("https://<your-database-name>.firebaseio.com/").unwrap();

    HttpServer::new(move || {
        App::new()
            .route("/data", web::get().to(get_data))
            .route("/data", web::post().to(post_data))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn get_data() -> impl Responder {
    // Get data from Firebase Realtime Database
    let data = firebase.get("your_data_key").await.unwrap();
    web::Json(data)
}

async fn post_data(item: web::Json<YourDataType>) -> impl Responder {
    // Post data to Firebase Realtime Database
    firebase.set("your_data_key", item.0).await.unwrap();
    "Data saved successfully"
}