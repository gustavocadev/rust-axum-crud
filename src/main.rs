use axum::{
  extract::{Path, State},
  routing::get,
  Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
struct User {
  id: u8,
  name: String,
  age: u8,
}

#[derive(Deserialize, Clone)]
struct CreateUser {
  name: String,
  age: u8,
}

#[derive(Deserialize, Clone)]
struct UpdateUser {
  name: Option<String>,
  age: Option<u8>,
}

type Users = Arc<Mutex<Vec<User>>>;

#[tokio::main]
async fn main() {
  let users = Arc::new(Mutex::new(vec![
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
  ]));

  let app = Router::new()
    .route("/users", get(get_users).post(create_user))
    .route(
      "/users/:id",
      get(get_user).delete(delete_user).patch(update_user),
    )
    .with_state(users);

  async fn get_users(State(users): State<Users>) -> Json<Vec<User>> {
    let users_read = users.lock().unwrap();
    Json(users_read.clone())
  }

  async fn get_user(State(users): State<Users>, Path(user_id): Path<u8>) -> Json<User> {
    let users = users.lock().unwrap();
    let user = users.iter().find(|user| user.id == user_id);

    if let Some(user) = user {
      return Json(user.clone());
    }

    Json(User {
      id: 0,
      name: "".to_string(),
      age: 0,
    })
  }

  async fn create_user(
    State(users): State<Users>,
    Json(body): Json<CreateUser>,
  ) -> Json<Vec<User>> {
    let mut users = users.lock().unwrap();
    let user_id = users.len() as u8 + 1;

    users.push(User {
      id: user_id,
      name: body.name,
      age: body.age,
    });

    Json(users.clone())
  }

  async fn delete_user(State(users): State<Users>, Path(user_id): Path<u8>) -> Json<Vec<User>> {
    let mut users = users.lock().unwrap();
    let user_idx = users.iter().position(|user| user.id == user_id);

    users.remove(user_idx.unwrap());

    Json(users.clone())
  }

  async fn update_user(
    State(users): State<Users>,
    Path(user_id): Path<u8>,
    Json(body): Json<UpdateUser>,
  ) -> Json<Vec<User>> {
    let users = users.lock().unwrap();

    if let Some(user) = users.clone().iter_mut().find(|user| user.id == user_id) {
      if let Some(name) = body.name {
        user.name = name;
      }

      if let Some(age) = body.age {
        user.age = age;
      }
    }

    Json(users.clone())
  }

  // listen
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap()
}
