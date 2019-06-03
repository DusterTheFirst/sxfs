use crate::HBS;
use handlebars::TemplateError;

#[derive(RustEmbed)]
#[folder = "src/views/templates"]
struct HBSTemplates;

#[derive(RustEmbed)]
#[folder = "src/views/partials"]
struct HBSPartials;

type Template = (String, Option<TemplateError>);

pub fn load_templates() -> impl Iterator<Item = Template> {
    HBSTemplates::iter().map(|template_file| {
        let mut regestry = HBS.lock().unwrap();
        let raw_file = &HBSTemplates::get(&template_file).unwrap();
        let file = std::str::from_utf8(raw_file).unwrap();
        let template_name = template_file.replace(".hbs", "");

        (template_name.clone(), regestry.register_template_string(&template_name, file).err())
    })
}

type Partial = (String, Option<TemplateError>);

pub fn load_partials() -> impl Iterator<Item = Partial>  {
    HBSPartials::iter().map(|partial_file| {
        let mut regestry = HBS.lock().unwrap();
        let raw_file = &HBSPartials::get(&partial_file).unwrap();
        let file = std::str::from_utf8(raw_file).unwrap();
        let partial_name = partial_file.replace(".hbs", "");

        (partial_name.clone(), regestry.register_partial(&partial_name, file).err())
    })
}