use crate::HBS;
use colored::*;
use handlebars::TemplateError;

#[derive(RustEmbed)]
#[folder = "src/views/templates"]
struct HBSTemplates;

#[derive(RustEmbed)]
#[folder = "src/views/partials"]
struct HBSPartials;

//TODO: Make into iterator?
pub fn load_templates() -> Result<(), TemplateError> {
    for template_file in HBSTemplates::iter() {
        let mut regestry = HBS.lock().unwrap();
        let raw_file = &HBSTemplates::get(&template_file).unwrap();
        let file = std::str::from_utf8(raw_file).unwrap();
        let template_name = template_file.replace(".hbs", "");

        print!("\r");
        match regestry.register_template_string(&template_name, file) {
            Ok(()) => debug!("{} - {}", template_file.blue(), "OK".green()),
            Err(reason) => {
                error!("{} - {}", template_file.blue(), "FAIL".red());
                trace!("{}", reason);
                return Err(reason);
            }
        }
    }

    Ok(())
}

pub fn load_partials() -> Result<(), TemplateError>  {
    for partial_file in HBSPartials::iter() {
        let mut regestry = HBS.lock().unwrap();
        let raw_file = &HBSPartials::get(&partial_file).unwrap();
        let file = std::str::from_utf8(raw_file).unwrap();
        let partial_name = partial_file.replace(".hbs", "");

        print!("\r");
        match regestry.register_partial(&partial_name, file) {
            Ok(()) => debug!("{} - {}", partial_file.blue(), "OK".green()),
            Err(reason) =>{
                error!("{} - {}", partial_file.blue(), "FAIL".red());
                trace!("{}", reason);
                return Err(reason);
            }
        }   
    }
    
    Ok(())
}