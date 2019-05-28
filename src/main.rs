#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;

use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;
use std::io::ErrorKind;
use std::io;
use std::sync::Mutex;

use handlebars::Handlebars;

use rocket::response::content::Content;
use rocket::http::ContentType;

lazy_static! {
    static ref reg: Mutex<Handlebars> = Mutex::new(Handlebars::new());
}

#[get("/")]
fn index() -> Content<String> {
    match fs::read_dir("uploads") {
        Err(why) => Content(ContentType::Plain, format!("! {:?}", why.kind())),
        Ok(paths) => {
            let paths_arr: Vec<PathBuf> = paths.collect::<Vec<io::Result<DirEntry>>>().iter().map(|x| x.as_ref().unwrap().path()).collect();
            Content(ContentType::HTML, reg.lock().unwrap().render("index", &json!({"dir": "uploads", "paths": paths_arr})).unwrap())
        },
    }
}

#[get("/f/<filename..>")]
fn file(filename: PathBuf) -> Content<String> {
    Content(ContentType::HTML, "not yet".to_owned())
}

fn main() -> std::io::Result<()> {
    println!("Running SXFS from {}", std::env::current_dir().unwrap().display());

    let uploads_dir = &Path::new("uploads");
    match fs::create_dir(uploads_dir) {
        Ok(_) => println!("Created upload directory {:?}", uploads_dir),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => println!("Found upload directory {:?}", uploads_dir),
            _ => return Err(e)
        }
    }

    // Regester templates
    let template_dir = &Path::new("views");
    reg.lock().unwrap().set_strict_mode(true);
    reg.lock().unwrap().register_templates_directory(".hbs", template_dir);

    println!("Regestered templates: {:?}", reg.lock().unwrap().get_templates().keys());

    rocket::ignite().mount("/", routes![index]).launch();

    Ok(())
}