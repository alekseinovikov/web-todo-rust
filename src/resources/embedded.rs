use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/components/"]
pub struct Components;

#[derive(RustEmbed)]
#[folder = "static/public/"]
pub struct Public;

#[derive(RustEmbed)]
#[folder = "static/templates/"]
pub struct Templates;
