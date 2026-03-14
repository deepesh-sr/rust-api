use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use dotenv;
use std::env;
use std::error::Error;
use chrono::{DateTime, Utc};

use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use sqlx::{FromRow, PgPool};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[derive(Serialize, FromRow,Debug)]
struct Project {
    id: i32,
    title: String,
    description: String,
    is_completed: bool,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct CreateProject {
    title : String, 
    description : String
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await?;
    
  

    let state = AppState{pool};
    let app = Router::new()
    .route("/", get(|| async { "Hello Deepesh" }))
    .route("/projects", get(get_projects).post(create_project))
    .route("/project/:id", get(get_project_by_id).delete(delete_project_by_id))
    .with_state(state);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4444").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_projects(State(state): State<AppState>)-> Result<Json<Vec<Project>> , StatusCode>{
  let projects =   sqlx::query_as::<_,Project>("SELECT * FROM projects").fetch_all(&state.pool).await.map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
  Ok(Json(projects))
}

async fn get_project_by_id(State(state): State<AppState>, Path(id): Path<i32>)->Result<Json<Project>, StatusCode>{
    let project = sqlx::query_as::<_,Project>("SELECT * FROM projects WHERE id= $1").bind(id).fetch_optional(&state.pool).await.map_err(|_| StatusCode::NO_CONTENT)?;
    match project { 
        Some(p) => Ok(Json(p)),
        None => Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_project_by_id(State(state): State<AppState>, Path(id): Path<i32>)->Result<StatusCode, StatusCode>{
    sqlx::query("DELETE FROM projects WHERE id = $1").bind(id).execute(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
   Ok(StatusCode::OK) 
}

async fn create_project(State(state):State<AppState> , input_query : Json<CreateProject>)->Result<StatusCode, StatusCode>{
    let title = &input_query.title;
    let description = &input_query.description;
    sqlx::query("INSERT INTO projects (title , description) VALUES ($1 , $2)").bind(title).bind(description).execute(&state.pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
   Ok(StatusCode::CREATED) 
}