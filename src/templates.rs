use askama::Template;

#[derive(Template)]
#[template(path = "form.html")]
pub struct FormTemplate;

#[derive(Template)]
#[template(path = "paste.html")]
pub struct PasteTemplate {
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub views: u32,
    pub id: String,
    pub language: String,
}
