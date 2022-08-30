#[macro_use] extern crate rocket;

mod routes;
use rocket_dyn_templates::Template;
use routes::{submit, file, root};

use rocket::{Rocket, Build};
use rocket::shield::{NoSniff, Shield};

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
            .attach(Shield::default().disable::<NoSniff>())
            .mount("/", routes![submit::route, file::route, root::route])
            .attach(Template::fairing())
}