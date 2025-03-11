use axum::{
    response::Html, routing::get, Router
};
use axum::extract::State;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;

use std::sync::Arc;

#[derive(FromRow, Debug, Clone)]
pub struct Level {
    pub name: String,
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

    // Fetch levels from db
    let levels = sqlx::query_as::<_, Level>("select name from obed_level;") 
        .fetch_all(&pool)
        .await
        .unwrap();

        let shared_state = Arc::new(levels);

        // Build app
        let app = Router::new().route("/", get(startup))
        .with_state(shared_state);

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    
}

async fn startup(State(state): State<Arc<Vec<Level>>>,) -> Html<String> {

    let mut names = vec![];

    for i in 0..state.len() {
        names.push(format!("<button> {} </button\n", state[i].name.to_string()));
    }
    

        // Read names from table
        let html = format!(
            "<h1>This is html! </h1>\n\
             <h2>HTML is so cool.</h2>\n
             {:?}", names
        );
    // create `Html` type like this
    Html(html)
}