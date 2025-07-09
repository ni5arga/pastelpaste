
use axum::{Router, Form, extract::Path, response::{Html, Redirect}, Extension};
use sqlx::SqlitePool;
use uuid::Uuid;
use pulldown_cmark::{Parser, html};
use time::OffsetDateTime;
use argon2::{Argon2, password_hash::{SaltString, PasswordHasher}};
use crate::{models::PasteForm, templates::*};

pub fn router(db: SqlitePool) -> Router {
    Router::new()
        .route("/", axum::routing::get(show_form).post(submit_paste))
        .route("/paste/:id", axum::routing::get(view_paste))
        .route("/raw/:id", axum::routing::get(raw_paste))
        .layer(Extension(db))
}

async fn show_form() -> Html<String> {
    let html = FormTemplate {}.render().unwrap();
    Html(html)
}

async fn submit_paste(Form(input): Form<PasteForm>, Extension(db): Extension<SqlitePool>) -> Redirect {
    let id = Uuid::new_v4().to_string()[..8].to_string();
    let now = OffsetDateTime::now_utc();
    let expires = match input.expire.as_str() {
        "1d" => Some(now + time::Duration::days(1)),
        "1w" => Some(now + time::Duration::weeks(1)),
        _ => None,
    };
    let password_hash = input.password.map(|pw| {
        let salt = SaltString::generate(&mut rand::thread_rng());
        Argon2::default().hash_password(pw.as_bytes(), &salt).unwrap().to_string()
    });
    sqlx::query!(
        r#"INSERT INTO pastes (id,title,content,created_at,expires_at,password) VALUES (?,?,?,?,?,?)"#,
        id, input.title, input.content,
        now.format(&time::format_description::well_known::Rfc3339).unwrap(),
        expires.map(|d| d.format(&time::format_description::well_known::Rfc3339).unwrap()),
        password_hash
    ).execute(&db).await.unwrap();

    Redirect::to(&format!("/paste/{}", id))
}

async fn view_paste(Path(id): Path<String>, Extension(db): Extension<SqlitePool>) -> Html<String> {
    let rec = sqlx::query!("SELECT * FROM pastes WHERE id = ?", id)
        .fetch_one(&db).await.unwrap();
    let mut content = String::new();
    html::push_html(&mut content, Parser::new(&rec.content));
    let html = PasteTemplate {
        title: rec.title,
        content_html: content,
        created_at: rec.created_at,
        views: rec.views + 1,
        id: rec.id.clone(),
    }.render().unwrap();
    sqlx::query!("UPDATE pastes SET views = views + 1 WHERE id = ?", id)
        .execute(&db).await.unwrap();
    Html(html)
}

async fn raw_paste(Path(id): Path<String>, Extension(db): Extension<SqlitePool>) -> String {
    let rec = sqlx::query!("SELECT content FROM pastes WHERE id = ?", id)
        .fetch_one(&db).await.unwrap();
    rec.content
}
