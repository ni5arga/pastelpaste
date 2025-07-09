use askama_axum::IntoResponse;
use askama::Template;
use axum::{
    extract::{Form, Path},
    response::{Redirect, Html},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use uuid::Uuid;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

mod templates;
use templates::{FormTemplate, PasteTemplate};

#[derive(Deserialize, Serialize, Clone)]
struct Paste {
    id: String,
    title: String,
    content: String,
    created_at: String,
    views: u32,
    language: String,
}

#[derive(Deserialize)]
struct PasteForm {
    title: String,
    content: String,
    language: String,
}

type PasteStore = Arc<RwLock<HashMap<String, Paste>>>;

#[tokio::main]
async fn main() {
    let store = load_pastes().await;
    let app = Router::new()
        .route("/", get(show_form).post(submit_paste))
        .route("/paste/:id", get(view_paste))
        .route("/raw/:id", get(view_raw))
        .route("/api/pastes", get(list_pastes))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(store);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("✨ PastelPaste running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn load_pastes() -> PasteStore {
    let file_path = "pastes.json";
    if let Ok(content) = fs::read_to_string(file_path) {
        if let Ok(pastes) = serde_json::from_str::<HashMap<String, Paste>>(&content) {
            return Arc::new(RwLock::new(pastes));
        }
    }
    Arc::new(RwLock::new(HashMap::new()))
}

async fn save_pastes(store: &PasteStore) {
    let pastes = store.read().await;
    if let Ok(json) = serde_json::to_string_pretty(&*pastes) {
        let _ = fs::write("pastes.json", json);
    }
}

async fn show_form() -> impl IntoResponse {
    Html(FormTemplate {}.render().unwrap())
}

async fn submit_paste(
    axum::extract::State(store): axum::extract::State<PasteStore>,
    Form(form): Form<PasteForm>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let now = OffsetDateTime::now_utc().format(&Rfc3339).unwrap();
    
    let paste = Paste {
        id: id.clone(),
        title: form.title,
        content: form.content,
        created_at: now,
        views: 0,
        language: form.language,
    };

    {
        let mut pastes = store.write().await;
        pastes.insert(id.clone(), paste);
    }
    
    save_pastes(&store).await;
    Redirect::to(&format!("/paste/{}", id))
}

async fn view_paste(
    axum::extract::State(store): axum::extract::State<PasteStore>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let pastes = store.read().await;
    
    if let Some(paste) = pastes.get(&id) {
        let mut updated_paste = paste.clone();
        updated_paste.views += 1;
        
        drop(pastes);
        {
            let mut pastes = store.write().await;
            pastes.insert(id.clone(), updated_paste.clone());
        }
        save_pastes(&store).await;

        let created_at_str = OffsetDateTime::parse(&updated_paste.created_at, &Rfc3339)
            .map(|dt| dt.format(&time::format_description::parse("%Y-%m-%d %H:%M UTC").unwrap()).unwrap())
            .unwrap_or_else(|_| updated_paste.created_at.clone());

        let html = PasteTemplate {
            title: updated_paste.title,
            content: updated_paste.content,
            created_at: created_at_str,
            views: updated_paste.views,
            id: updated_paste.id,
            language: updated_paste.language,
        };

        Html(html.render().unwrap())
    } else {
        Html("<h1>Paste not found</h1>".to_string())
    }
}

async fn view_raw(
    axum::extract::State(store): axum::extract::State<PasteStore>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let pastes = store.read().await;
    
    if let Some(paste) = pastes.get(&id) {
        Html(paste.content.clone())
    } else {
        Html("Paste not found".to_string())
    }
}

async fn list_pastes(
    axum::extract::State(store): axum::extract::State<PasteStore>,
) -> impl IntoResponse {
    let pastes = store.read().await;
    let pastes_vec: Vec<Paste> = pastes.values().cloned().collect();
    axum::Json(pastes_vec)
}
