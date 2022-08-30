use std::path::Path;

use regex::Regex;
use rocket::{form::Form, response::Redirect, fs::{TempFile, FileName}};
use uuid::Uuid;


#[derive(FromForm)]
pub struct FileUpload<'r> {
    user_file: TempFile<'r>
}

fn get_ascii_extension(filename: &FileName) -> &str {
    let true_name = filename.dangerous_unsafe_unsanitized_raw().as_str();
    let regex = Regex::new(r"(\.[ -~]+)$").expect("a valid regex");
    match regex.captures(true_name).map(|c| c.get(1)) {
        Some(Some(result)) => result.as_str(),
        Some(None) => "",
        None => ""
    }
}

fn generate_identifier(file: &TempFile) -> String {
    let id = Uuid::new_v4();
    let ext = file.raw_name().map(|f| get_ascii_extension(f)).unwrap_or_default();
    format!("{id}{ext}", id = id, ext = ext)
}


#[post("/submit", data = "<input>")]
pub async fn submit(mut input: Form<FileUpload<'_>>) -> std::io::Result<Redirect> {
    let filename = generate_identifier(&input.user_file);
    input.user_file.copy_to(Path::new(&format!("data/userfiles/{}", &filename))).await?;
    Ok(Redirect::to(uri!(crate::file(filename))))
}

