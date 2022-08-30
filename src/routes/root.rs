use std::collections::HashMap;

use rocket_dyn_templates::{context, Template};


#[get("/?<filename>")]
pub async fn route(filename: Option<&str>) -> Template {
    match filename {
        Some(name) => Template::render("index", context! {
            filename: name,
        }),
        None => Template::render("index", &(HashMap::new() as HashMap<&str, &str>))
    }
}