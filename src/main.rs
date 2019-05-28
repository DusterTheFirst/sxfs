#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rust_embed;

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

#[derive(RustEmbed)]
#[folder = "src/views/templates"]
struct HBSTemplates;

#[derive(RustEmbed)]
#[folder = "src/views/partials"]
struct HBSPartials;

lazy_static! {
    static ref HBS: Mutex<Handlebars> = Mutex::new(Handlebars::new());
}

#[get("/")]
fn index() -> Content<String> {
    match fs::read_dir("uploads") {
        Err(why) => Content(ContentType::Plain, format!("! {:?}", why.kind())),
        Ok(paths) => {
            let paths_arr: Vec<PathBuf> = paths.collect::<Vec<io::Result<DirEntry>>>().iter().map(|x| x.as_ref().unwrap().path()).collect();
            Content(ContentType::HTML, HBS.lock().unwrap().render("index", &json!({"dir": "uploads", "paths": paths_arr})).unwrap())
        },
    }
}

#[get("/f/<filename..>")]
fn view_file(filename: PathBuf) -> Content<String> {
    Content(ContentType::HTML, format!("{}. cool", filename.display()))
}

fn main() -> std::io::Result<()> {
    // TODO: Config
    println!("Running SXFS from {}", std::env::current_dir().unwrap().display());

    let uploads_dir = &Path::new("uploads");
    match fs::create_dir(uploads_dir) {
        Ok(()) => println!("Created upload directory {:?}", uploads_dir),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => println!("Found upload directory {:?}", uploads_dir),
            _ => return Err(e)
        }
    }

    // TODO: User defined templates
    // Regester templates
    HBS.lock().unwrap().set_strict_mode(true);
    println!("Loading templates...");
    for template_file in HBSTemplates::iter() {
        print!("{}... ", template_file);
        let mut regestry = HBS.lock().unwrap();
        let raw_file = &HBSTemplates::get(&template_file).unwrap();
        let file = std::str::from_utf8(raw_file).unwrap();
        let template_name = template_file.replace(".hbs", "");
        match regestry.register_template_string(&template_name, file) {
            Ok(()) => println!("OK"),
            Err(reason) => println!("FAIL {:#?}", reason)
        }
    }
    println!("Loading partials...");
    for partial_file in HBSPartials::iter() {
        print!("{}... ", partial_file);
        let mut regestry = HBS.lock().unwrap();
        let raw_file = &HBSPartials::get(&partial_file).unwrap();
        let file = std::str::from_utf8(raw_file).unwrap();
        let partial_name = partial_file.replace(".hbs", "");
        match regestry.register_partial(&partial_name, file) {
            Ok(()) => println!("OK"),
            Err(reason) => println!("FAIL {:#?}", reason)
        }
        
    }
    // let template_dir = &Path::new("views");
    // HBS.lock().unwrap().register_templates_directory(".hbs", template_dir);

    // println!("Regestered templates: {:?}", HBS.lock().unwrap().get_templates().keys());


    rocket::ignite().mount("/", routes![index, view_file]).launch();

    Ok(())
}