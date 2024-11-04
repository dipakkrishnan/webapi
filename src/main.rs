use axum::{
    extract::State,
    routing::get,
    routing::post,
    Router,
    extract::Path,
    Json
};
use clap::Parser;
mod db;
use db::Database;
mod models;
use models::{CreateUserRequest, DeleteUserRequest, UpdateUserRequest};
use std::sync::Arc;


#[derive(Parser)]
struct Args {
    appname: String,
    port: String,
}

struct AppState {
    db: Database
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let appname = args.appname; 
    let port = args.port;
    let dbname = format!("{}.db", appname);
    let db = Database::new(&dbname).unwrap();
    let state = Arc::new(AppState { db });
    let endpoint = format!("0.0.0.0:{}", port);

    // define api
    let app = Router::new()
        .route("/", get(|| async {
            format!("Welcome to a simple web app!") 
        }))
        .route("/users/create", post(create_user))
        .route("/users/delete", post(delete_user))
        .route("/users/update", post(update_user))
        .route("/users/:username", get(get_user))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(&endpoint).await.unwrap();
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

async fn delete_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeleteUserRequest>,
) -> String {
    let name = &payload.name;
    if let Err(e) = state.db.delete_user(name).await {
        eprintln!("Failed to add user: {}", e);
        return "Failed to create user".to_string();
    }
    format!("User {} deleted successfully", name)
}

async fn update_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateUserRequest>,
) -> String {
    let name = &payload.name;
    if let Err(e) = state.db.update_user(name, &payload.email).await {
        eprintln!("Failed to update user metadata: {}", e);
        return "Failed to update user metadata".to_string();
    }
    format!("User {} metadata updated successfully", name)
}

async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> String {
    //let Err(r) = state.db.get_user(&username).await;
    //format!("User {} metadata updated successfully", name)
}
