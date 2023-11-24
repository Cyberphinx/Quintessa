use axum::{
    http::{HeaderName, HeaderValue, Method},
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, COOKIE, SET_COOKIE, WWW_AUTHENTICATE};
use std::env;
use tower_http::cors::CorsLayer;

use crate::{
    app_state::AppState,
    controllers::{
        hello_world::{hello_admin_auth, hello_world, hello_world_auth},
        media_controller::{create_media, delete_media_endpoint, edit_media, list_media},
        projects_controller::{
            create_project, delete_project_endpoint, edit_project, get_project, get_projects,
            get_totals,
        },
        resumes_controller::{create_education, create_resume, create_work, get_resume},
        users_controller::{create_user, edit_role_endpoint, login, logout},
    },
    middleware::require_authentication::{require_admin_authentication, require_authentication},
};

// build the router
pub fn create_router(app_state: AppState) -> Router {
    let client_url = env::var("CLIENT_URL").unwrap();
    let client_url_dns = env::var("CLIENT_URL_DNS").unwrap();
    let client_url_dns_www = env::var("CLIENT_URL_DNS_WWW").unwrap();
    println!(
        "Cors allowed client URL: {}, {}, {}",
        &client_url, &client_url_dns, &client_url_dns_www
    );

    let allowed_origins = vec![
        client_url.parse::<HeaderValue>().unwrap(),
        client_url_dns.parse::<HeaderValue>().unwrap(),
        client_url_dns_www.parse::<HeaderValue>().unwrap(),
    ];
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(allowed_origins)
        .allow_headers(vec![
            AUTHORIZATION,
            COOKIE,
            CONTENT_TYPE,
            SET_COOKIE,
            WWW_AUTHENTICATE,
        ])
        .allow_credentials(true)
        .expose_headers([
            WWW_AUTHENTICATE,
            "pagination".parse::<HeaderName>().unwrap(),
            SET_COOKIE,
            COOKIE,
            CONTENT_TYPE,
            AUTHORIZATION,
        ]);

    let admin_router = Router::new()
        .route("/projects/:project_id", delete(delete_project_endpoint))
        .route("/projects/:project_id", put(edit_project))
        .route("/projects", post(create_project))
        .route("/media", put(edit_media))
        .route("/media/:media_id", delete(delete_media_endpoint))
        .route("/media", get(list_media))
        .route("/media", post(create_media))
        .route("/work", post(create_work))
        .route("/education", post(create_education))
        .route("/resume", post(create_resume))
        .route("/users", post(create_user))
        .route("/users/role", post(edit_role_endpoint))
        .route("/hello_admin", get(hello_admin_auth))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_admin_authentication,
        ));

    let auth_router = Router::new()
        .route("/resume/:resume_id", get(get_resume))
        .route("/projects", get(get_projects))
        .route("/projects/totals", get(get_totals))
        .route("/projects/:project_id", get(get_project))
        .route("/users/logout", post(logout))
        .route("/hello_auth", get(hello_world_auth))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ));

    Router::new()
        .nest("/api", admin_router)
        .nest("/api", auth_router)
        .route("/api", get(hello_world))
        .route("/api/users/login", post(login))
        .with_state(app_state)
        .layer(cors)
}
