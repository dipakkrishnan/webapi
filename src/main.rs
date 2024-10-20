use axum::{
    extract::State,
    routing::get,
    routing::post,
    Router,
    Json,
};
use clap::Parser;
mod db;
use db::Database;
mod models;
use models::{CreateUserRequest};
use std::sync::Arc;


#[derive(Parser)]
struct Args {
    appname: String,
}

struct AppState {
    db: Database
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let appname = args.appname; 
    let dbname = format!("{}.db", appname);
    let db = Database::new(&dbname).unwrap();
    let state = Arc::new(AppState { db });

    // define api
    let app = Router::new()
        .route("/", get(|| async {
            format!("Welcome to a simple web app!") 
        }))
        .route("/users", post(create_user))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> String {
    let name = &payload.name;
    if let Err(e) = state.db.add_user(name, &payload.email).await {
        eprintln!("Failed to add user: {}", e);
        return "Failed to create user".to_string();
    }
    format!("User {} created successfully", name)
}
