#[macro_use] extern crate rocket;
use rocket::fs::TempFile;
use uuid::Uuid;
use std::path::Path;

#[post("/upload", data = "<file>")]
async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
    let id = Uuid::new_v4();
    let form = format!("./files/{}", id.to_string());
    let path = Path::new(&form);
    file.persist_to(path).await?;
    Ok(())
}

#[get("/download/<identifier>")]
fn download(identifier: &str) -> &'static str {
    "You did a heckin upload fam!"
}

#[delete("/delete/<identifier>")]
fn delete(identifier: &str) -> &'static str {
    "OMG i just deleted!"
}

#[put("/replace/<identifier>")]
fn replace(identifier: &str) -> &'static str {
    "I just replaced a file!"
}

#[get("/list")]
fn list() -> &'static str {
    "Here are all the files!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/v1", routes![upload,download,delete,replace,list])
}
