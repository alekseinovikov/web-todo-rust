use rocket_dyn_templates::tera::Tera;

use crate::resources::embedded::Templates;

pub fn customize(tera: &mut Tera) {
    for file_file_path in Templates::iter() {
        if let Some(file) = Templates::get(file_file_path.as_ref()) {
            let content = std::str::from_utf8(file.data.as_ref()).unwrap();
            let template_name = file_file_path.as_ref();

            tera.add_raw_template(template_name, content)
                .expect("Failed to add template to Tera.")
        }
    }
}