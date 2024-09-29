use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize, Clone)]
struct User {
  id: u8,
  name: String,
  age: u8,
}

type Users = Arc<RwLock<Vec<User>>>;

#[tokio::main]
async fn main() {
  let users = vec![
    User {
      id: 1,
      name: "John".to_string(),
      age: 32,
    },
    User {
      id: 2,
      name: "Jane".to_string(),
      age: 28,
    },
  ];
  // Envolver los usuarios en Arc y Mutex de Tokio
  let users: Users = Arc::new(RwLock::new(serde_json::from_value(json!(users)).unwrap()));

  let app = Router::new()
    .route("/users", get(get_users))
    .with_state(users);

  async fn get_users(State(users): State<Users>) -> Json<Vec<User>> {
    let users_read = users.read().unwrap();
    Json(users_read.clone())
  }

  // listen
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap()
}
