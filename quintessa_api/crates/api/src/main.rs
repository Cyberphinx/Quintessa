use api::{app_state::AppState, migrations::Migrator, run, utilities::token_wrapper::TokenWrapper};
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use std::env;

#[tokio::main]
async fn main() {
    // get environment variables from .env file
    let dot_env = dotenvy::dotenv();
    let database_url = match dot_env {
        Ok(_) => env::var("DATABASE_URL").expect("to have database url"),
        Err(_) => {
            //fly io
            env::var("DATABASE_URL")
                .expect("to have database url")
                .replace("flycast", "internal")
        }
    };
    // println!("Database url: {}", &database_url);

    let app_env = env::var("ENVIRONMENT").unwrap();
    println!("Running in {} mode!", &app_env);

    let jwt_secret = env::var("JWT_SECRET").unwrap();

    // connect to the database
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    // Apply all pending migrations
    match Migrator::up(&db, None).await {
        Err(error) => {
            eprintln!(
                "Error creating pending migration to the database: {:?}",
                error
            );
            panic!();
        }
        Ok(_) => println!("Successfully applied pending migration to the database!"),
    };

    // Drop all tables from the database, then reapply all migrations
    // match Migrator::fresh(&db).await {
    //     Err(error) => {
    //         eprintln!(
    //             "Error creating complete fresh migration to the database: {:?}",
    //             error
    //         );
    //         panic!();
    //     }
    //     Ok(_) => println!("Successfully applied fresh migration to the database!"),
    // };

    // Roll back the last applied migration
    // let migrations = 2;
    // match Migrator::down(&db, Some(migrations)).await {
    //     Err(error) => {
    //         eprintln!(
    //             "Error rolling back the last {} migration to the database: {:?}",
    //             migrations, error
    //         );
    //         panic!();
    //     }
    //     Ok(_) => println!(
    //         "Successfully rolled back the last {} migration to the database!",
    //         migrations
    //     ),
    // }

    // instantiate a http client
    let http_client = match reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36")
        .build() {
            Ok(http_client) => http_client,
            Err(error) => {
                eprintln!("Error building reqwest http client: {:?}", error);
                panic!();
        }
    };

    let app_state = AppState {
        http_client,
        db,
        jwt_secret: TokenWrapper(jwt_secret),
    };

    run(app_state).await;
}
