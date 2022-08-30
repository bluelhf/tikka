use std::{path::Path, time::{SystemTime, UNIX_EPOCH}};

use super::root;
use regex::Regex;
use rocket::{form::Form, response::Redirect, fs::{TempFile, FileName}};
use uuid::{Uuid, v1::{Context, Timestamp}};


#[derive(FromForm)]
pub struct FileUpload<'r> {
    user_file: TempFile<'r>
}

fn get_ascii_extension(filename: &FileName) -> &str {
    let true_name = filename.dangerous_unsafe_unsanitized_raw().as_str();
    let regex = Regex::new(r"(\.[ -~]{0,10})$").expect("a valid regex");
    match regex.captures(true_name).map(|c| c.get(1)) {
        Some(Some(result)) => result.as_str(),
        Some(None) => "",
        None => ""
    }
}

const CONTEXT: Context = Context::new(7175);

fn generate_uuid() -> Uuid {
    let system_time = SystemTime::now();
    let unix_time = system_time.duration_since(UNIX_EPOCH)
            .expect("a valid unix timestamp");

    let timestamp = Timestamp::from_unix(CONTEXT, 
        unix_time.as_secs(), unix_time.subsec_nanos());
    Uuid::new_v1(timestamp, &[8, 0, 0, 0, 8, 5])
}

fn generate_identifier(file: &TempFile) -> String {
    let id = generate_uuid();
    let ext = file.raw_name().map(|f| get_ascii_extension(f)).unwrap_or_default();
    format!("{id}{ext}", id = id, ext = ext)
}

#[post("/submit", data = "<input>")]
pub async fn route(mut input: Form<FileUpload<'_>>) -> std::io::Result<Redirect> {
    let filename = generate_identifier(&input.user_file);
    input.user_file.copy_to(Path::new(&format!("data/userfiles/{}", &filename))).await?;
    Ok(Redirect::to(uri!(root::route(Some(filename)))))
}

