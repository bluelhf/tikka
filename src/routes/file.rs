use rocket::{tokio::fs::File, Response, Request, response, response::Responder};

pub struct Attachment(File);


impl<'r> Responder<'r, 'r> for Attachment {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'r> {
        Response::build_from(self.0.respond_to(req)?)
        .raw_header("Content-Disposition", "attachment")
        .ok()
    }
}

#[get("/file/<filename>")]
pub async fn route(filename: &str) -> std::io::Result<Attachment> {
    let path = format!("data/userfiles/{name}", name = filename);
    File::open(path).await.map(|f| Attachment(f))
}
