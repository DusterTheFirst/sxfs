#![feature(proc_macro_hygiene, decl_macro, type_alias_enum_variants)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rust_embed;

use std::path::{Path};
use std::io::ErrorKind;
use std::sync::Mutex;
use std::fs;

use handlebars::Handlebars;

#[derive(RustEmbed)]
#[folder = "src/views/templates"]
struct HBSTemplates;

#[derive(RustEmbed)]
#[folder = "src/views/partials"]
struct HBSPartials;

lazy_static! {
    pub static ref HBS: Mutex<Handlebars> = Mutex::new(Handlebars::new());
}

mod gaurds {
    use rocket::request::{Request, FromRequest, Outcome};
    use rocket::http::Status;

    pub struct AuthGaurd(String);

    #[derive(Debug)]
    pub enum AuthGaurdError {
        Missing,
        Invalid,
        BadCount
    }

    impl AuthGaurd {
        pub fn is_valid(token: &str) -> bool {
            token == "token"
        }
    }

    impl<'a, 'r> FromRequest<'a, 'r> for AuthGaurd {
        type Error = AuthGaurdError;

        fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
            let tokens: Vec<_> = request.headers().get("Authorization").collect();

            match tokens.len() {
                0 => Outcome::Failure((Status::Unauthorized, AuthGaurdError::Missing)),
                1 if Self::is_valid(tokens[0]) => Outcome::Success(AuthGaurd(tokens[0].to_string())),
                1 => Outcome::Failure((Status::Unauthorized, AuthGaurdError::Invalid)),
                _ => Outcome::Failure((Status::BadRequest, AuthGaurdError::BadCount)),
            }
        }
    }
}

mod paths {
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


    rocket::ignite().register(catchers![
        paths::not_found
    ]).mount("/", routes![
        paths::index,
        paths::view_upload,
        paths::redirect_short_url,
        paths::make_upload
    ]).launch();

    Ok(())
}