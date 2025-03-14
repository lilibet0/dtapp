use axum::extract::State;
use axum::{Router, response::Html, routing::get};
use sqlx::FromRow;
use sqlx::postgres::PgPoolOptions;

use std::sync::Arc;

#[derive(FromRow, Debug, Clone)]
pub struct Exercise {
    pub exercise_name: String,
}

#[derive(Clone)]
struct SharedState {
    pool: sqlx::Pool<sqlx::Postgres>
}

#[tokio::main]
async fn main() {
    // Connect to local db
    let pool = PgPoolOptions::new()
        .max_connections(5)
        // use your own credentials
        .connect("postgres://dtapp@localhost/dtappdb")
        .await
        .expect("couldn't connect to the database");

        let shared_state = Arc::new(SharedState {pool});

    // Build app
    let app = Router::new()
        .route("/", get(index))
        .route("/novice", get(novicehandler))
        .with_state(shared_state);

    // listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../BlueRibbon.html"))
}

async fn novicehandler(State(shared_state): State<Arc<SharedState>>) -> Html<String> {

    let exercises = getexercises("Novice", shared_state.pool.clone())
    .await;

    let mut exercisenames = vec![];

    for name in exercises.iter() {
        exercisenames.push(name.exercise_name.to_string());
    }
        // Read names from table
        let html = format!(
            "<h1>Novice Exercises </h1>\n\
             {:?}", exercisenames
        );
    Html(html)
}

async fn getexercises(level: &str, pool: sqlx::Pool<sqlx::Postgres>) -> Vec<Exercise> {
    let query = format!(
        "SELECT exercise_name
        FROM exercise
        WHERE class_name = '{}';
    ", level);

    let exercises = sqlx::query_as::<_, Exercise>(&query)
        .fetch_all(&pool)
        .await
        .unwrap();

    return exercises;
}