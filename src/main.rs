#[macro_use] extern crate rocket;

mod endpoints;

use rocket::{Rocket, Build};
use rocket::tokio::fs::File;
use rocket::shield::{NoSniff, Shield};
use rocket::fs::{NamedFile};


#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("data/index.html").await.ok()
}

#[get("/file/<filename>")]
async fn file(filename: &str) -> std::io::Result<File> {
    let path = format!("data/userfiles/{name}", name = filename);
    File::open(path).await
}


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
            .attach(Shield::default().disable::<NoSniff>())
            .mount("/", routes![index, endpoints::submit::submit, file])
}