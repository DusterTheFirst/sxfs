use std::path::PathBuf;
use std::fs;
use rocket::http::{ContentType, RawStr};
use rocket::response::{content::Content};
use rocket::Request;
use crate::HBS;
use crate::gaurds;

#[catch(404)]
pub fn not_found(req: &Request) -> Content<String> {
    Content(ContentType::HTML, HBS.lock().unwrap().render("404", &json!({"uri": req.uri().path()})).unwrap())
}
#[catch(401)]
pub fn unauthorized(req: &Request) -> Content<String> {
    Content(ContentType::HTML, HBS.lock().unwrap().render("401", &json!({"uri": req.uri().path(), "method": req.method().as_str(), "reason": format!("{:?}", req.guard::<gaurds::AuthGaurd>().failed())})).unwrap())
}

#[get("/")]
pub fn index() -> Content<String> {
    match fs::read_dir("uploads") {
        Err(why) => Content(ContentType::Plain, format!("! {:?}", why.kind())),
        Ok(paths) => {
            let paths_arr: Vec<PathBuf> = paths.collect::<Vec<_>>().iter().map(|x| x.as_ref().unwrap().path()).collect();
            Content(ContentType::HTML, HBS.lock().unwrap().render("index", &json!({"dir": "uploads", "paths": paths_arr})).unwrap())
        },
    }
}

#[get("/u/<filename..>")]
pub fn view_upload(filename: PathBuf) -> String {
    format!("FILE\n\nFILENAME: {:?}", filename)
}
#[post("/upload")]
pub fn make_upload(auth: gaurds::AuthGaurd) -> String {
    format!("you will upload")
}
#[get("/r/<id>")]
pub fn redirect_short_url(id: &RawStr) -> String {
    format!("URL REDIRECT\n\nID: {}", id)
}