use rocket::get;
// use rocket::request::FlashMessage;
use rocket::State;
use rocket_contrib::templates::Template;
use wqms::Workspace;
/// Wrapper around the Markdown parser and renderer to render markdown
// fn render_markdown(text: &str) -> String {
//     use comrak::{markdown_to_html, ComrakOptions};

//     let options = {
//         let mut options = ComrakOptions::default();
//         options.safe = true;
//         options.ext_superscript = true;
//         options.ext_table = true;
//         options.ext_autolink = true;
//         options.ext_tasklist = true;
//         options.ext_strikethrough = true;
//         options
//     };

//     markdown_to_html(text, &options)
// }

#[get("/")]
pub fn index(state: State<Workspace>) -> Template {
    Template::render("index", "name")
}
