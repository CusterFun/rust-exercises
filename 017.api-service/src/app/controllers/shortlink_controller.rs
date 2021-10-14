use crate::app::models::dto;
use crate::app::models::shortlink;
use axum::extract::Extension;
use axum::http::header::LOCATION;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{extract, Json};
use sqlx::{MySql, Pool};

pub async fn create_shortlink(
    Json(req): Json<dto::CreateShortLinkReq>,
    Extension(pool): Extension<Pool<MySql>>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    match shortlink::create_shortlink(&pool, &req.url).await {
        Ok(_) => (StatusCode::OK, Json(dto::CreateShortLinkResp { ok: true })),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(dto::CreateShortLinkResp { ok: false }),
        ),
    }
}

pub async fn delete_shortlink(
    Json(req): Json<dto::DeleteShortLinkReq>,
    Extension(pool): Extension<Pool<MySql>>,
) -> impl IntoResponse {
    println!("{:#?}", req);
    match shortlink::delete_shortlink(&pool, req.id).await {
        Ok(_) => (StatusCode::OK, Json(dto::DeleteShortLinkResp { ok: true })),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(dto::DeleteShortLinkResp { ok: false }),
        ),
    }
}

pub async fn get_shortlink(
    extract::Path(id): extract::Path<i32>,
    Extension(pool): Extension<Pool<MySql>>,
) -> impl IntoResponse {
    let mut url = "/api/not_found.json";
    match shortlink::get_shortlink(&pool, id).await {
        Ok(record) => {
            url = Box::leak(record.url.into_boxed_str());
        }
        Err(err) => {
            println!("err={:#?}", err);
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, url.parse().unwrap());
    (StatusCode::FOUND, headers, ())
}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::OK, "404 Not Found")
}
