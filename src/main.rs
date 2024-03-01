#[macro_use] extern crate rocket;


use serde;
use rocket::serde::json::Json;
use rocket::{get, post, routes, data};
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Cors, CorsOptions,
};
use tokio_postgres::{NoTls, Error};


async fn connect_to_db() -> Result<tokio_postgres::Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=miriam dbname=kopesha",
        NoTls,
    ).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    match client.execute("CREATE TABLE IF NOT EXISTS clients(\
    client_id SERIAL PRIMARY KEY,\
    client_name varchar(50),\
    client_workplace varchar(100),\
    client_position varchar(20),\
    client_amount int,\
    client_contract varchar(255),\
    client_rate int\
    )", &[]).await {
        Ok(res) => println!("Table results: {res}"),
        Err(e) => println!("Error: {}", e)
    }

    Ok(client)
}

#[derive(serde::Deserialize)]
struct Client {
    client_name: String,
    client_workplace: String,
    client_position: String,
    client_amount: u32,
    client_contract: String,
    client_rate: u32,
}



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/register-client", data = "<client>")]
async fn register_client(client: Json<Client>, db_client: data::Data<'_>) -> String {
    // if let Err(e) = save_user_to_db(&user.into_inner(), &db_client.get_ref()).await {
    //     return format!("Failed to save user: {}", e);
    // }
    "User saved successfully".into()
}


#[launch]
async fn rocket() -> _ {
    let db_client = connect_to_db().await.expect("Error connecting to database");

    let allowed_origin = "http://localhost:5173";
    let allowed_origin2 = "http://192.168.1.133:5173";

    let allowed_origins = AllowedOrigins::some_exact(&[allowed_origin, allowed_origin2]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors().expect("Error creating CORS configuration");

    rocket::build()
        // .attach(Config {
        //     limits: limits,
        //     ..Config::default()
        // })
        .attach(cors)
        // .manage(limits)
        .manage(db_client)
        .mount("/", routes![index])
}