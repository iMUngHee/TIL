#[macro_use]
extern crate rocket;

use std::{error::Error, time::SystemTime};

use rocket::{
    fairing::{self, AdHoc, Fairing, Info, Kind},
    http::Method,
    log::private::Record,
    serde::json::Json,
    Build, Request, Response, Rocket,
};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use serde::{Deserialize, Serialize};

use rocket_db_pools::{sqlx, Connection, Database};

use futures::future::TryFutureExt;

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Database)]
#[database("db")]
struct Db(sqlx::SqlitePool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

#[get("/")]
fn index() -> String {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    format!("Now: {}", now)
}

#[derive(Deserialize, Serialize, Clone)]
struct User {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    username: String,
}

#[post("/auth", data = "<user>")]
async fn hello(mut db: Connection<Db>, user: Json<User>) -> Result<Json<User>> {
    let result = sqlx::query!("INSERT INTO users (username) VALUES (?)", user.username,)
        .execute(&mut *db)
        .await?;

    Ok(Json(User {
        id: Some(result.last_insert_rowid()),
        username: user.username.clone(),
    }))
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct ReqMsg {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    user_id: i64,
    text: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Msg {
    id: i64,
    user_id: i64,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct VecMsg {
    data: Vec<Msg>,
}

#[post("/msg", data = "<msg>")]
async fn write_msg(mut db: Connection<Db>, msg: Json<ReqMsg>) -> Result<Json<Msg>> {
    let result = sqlx::query!(
        "INSERT INTO messages (user_id, text) VALUES (?, ?)",
        msg.user_id,
        msg.text,
    )
    .execute(&mut *db)
    .await?;

    Ok(Json(Msg {
        id: result.last_insert_rowid(),
        user_id: msg.user_id.clone(),
        text: msg.text.clone(),
    }))
}

#[get("/<id>")]
async fn read(mut db: Connection<Db>, id: i64) -> Result<Json<VecMsg>> {
    let result = sqlx::query_as!(
        Msg,
        "SELECT id, user_id, text FROM messages WHERE user_id = ?",
        id
    )
    .fetch_all(&mut *db)
    .await
    .unwrap();
    // .map(|r| Msg {
    //     id: Some(r.id),
    //     user_id: r.user_id,
    //     text: r.text,
    // });

    println!("{:?}", result);

    Ok(Json(VecMsg { data: result }))
}

fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
    })
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::some_exact(&["http://127.0.0.1:8080"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        // allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    let _ = rocket::build()
        .mount("/", routes![index, hello, write_msg, read])
        .attach(stage())
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
