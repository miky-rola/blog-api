use actix_web::{web, HttpResponse, Responder};
// use serde_json::json;
use uuid::Uuid;
use serde::Serialize;

use crate::models::{User, Blog, Comment, Like};
use crate::orm::{create_user, create_blog, create_comment, create_like, get_user, get_blog, update_blog, delete_blog, get_comment, update_comment, delete_comment, get_like};
use crate::db::DbPool;
use crate::api_response::ApiResponse;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/users", web::post().to(create_user_handler))
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/blogs", web::post().to(create_blog_handler))
            .route("/blogs/{id}", web::get().to(get_blog_by_id))
            .route("/blogs/{id}", web::put().to(update_blog_by_id))
            .route("/blogs/{id}", web::delete().to(delete_blog_by_id))
            .route("/comments", web::post().to(create_comment_handler))
            .route("/comments/{id}", web::get().to(get_comment_by_id))
            .route("/comments/{id}", web::put().to(update_comment_handler))
            .route("/comments/{id}", web::delete().to(delete_comment_handler))
            .route("/likes", web::post().to(create_like_handler))
            .route("/likes/{id}", web::get().to(get_like_by_id))
    );
}

fn handle_diesel_result<T: Serialize>(result: Result<T, diesel::result::Error>) -> HttpResponse {
    match result {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::success(data)),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Database error".to_string())),
    }
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = User,
    responses(
        (status = 200, description = "User created successfully", body = User),
        (status = 500, description = "Internal server error")
    ),
    tag = "users"
)]
async fn create_user_handler(user: web::Json<User>, pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        create_user(&mut conn, &user.username, &user.email, &user.password_hash)
    }).await;
    
    match result {
        Ok(user_result) => handle_diesel_result(user_result),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Server error".to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    tag = "users"
)]
async fn get_user_by_id(user_id: web::Path<Uuid>, pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        get_user(&mut conn, user_id.into_inner())
    }).await;
    
    match result {
        Ok(user_result) => handle_diesel_result(user_result),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Server error".to_string())),
    }
}

#[utoipa::path(
    post,
    path = "/blogs",
    request_body = Blog,
    responses(
        (status = 200, description = "Blog created successfully", body = Blog),
        (status = 500, description = "Internal server error")
    ),
    tag = "blogs"
)]
async fn create_blog_handler(blog: web::Json<Blog>, pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        create_blog(&mut conn, &blog.title, &blog.content, blog.author_id)
    }).await;
    
    match result {
        Ok(blog_result) => handle_diesel_result(blog_result),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Server error".to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/blogs/{id}",
    responses(
        (status = 200, description = "Blog found", body = Blog),
        (status = 404, description = "Blog not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Blog ID")
    ),
    tag = "blogs"
)]
async fn get_blog_by_id(blog_id: web::Path<Uuid>, pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        get_blog(&mut conn, blog_id.into_inner())
    }).await;
    
    match result {
        Ok(blog_result) => handle_diesel_result(blog_result),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Server error".to_string())),
    }
}

#[utoipa::path(
    put,
    path = "/blogs/{id}",
    request_body = Blog,
    responses(
        (status = 200, description = "Blog updated successfully", body = Blog),
        (status = 404, description = "Blog not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = Uuid, Path, description = "Blog ID")
    ),
    tag = "blogs"
)]
async fn update_blog_by_id(blog_id: web::Path<Uuid>, blog: web::Json<Blog>, pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        update_blog(&mut conn, blog_id.into_inner(), &blog.title, &blog.content)
    }).await;
    
    match result {
        Ok(blog_result) => handle_diesel_result(blog_result),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Server error".to_string())),
    }
}

#[utoipa::path(
    delete,
    path = "/blogs/{id}",
    responses(
        (status = 200, description = "Blog deleted successfully"),
        (status = 404, description = "Blog not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = Uuid, Path, description = "Blog ID")
    ),
    tag = "blogs"
)]
async fn delete_blog_by_id(blog_id: web::Path<Uuid>, pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        delete_blog(&mut conn, blog_id.into_inner())
    }).await;
    
    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success(())),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Server error".to_string())),
    }
}

#[utoipa::path(
    post,
    path = "/comments",
    request_body = Comment,
    responses(
        (status = 200, description = "Comment created successfully", body = Comment),
        (status = 500, description = "Internal server error")
    ),
    tag = "comments"
)]
async fn create_comment_handler(comment: web::Json<Comment>, pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        create_comment(&mut conn, comment.blog_id, comment.user_id, &comment.content, comment.parent_comment_id)
    }).await;
    
    match result {
        Ok(comment_result) => handle_diesel_result(comment_result),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Server error".to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/comments/{id}",
    responses(
        (status = 200, description = "Comment found", body = Comment),
        (status = 404, description = "Comment not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Comment ID")
    ),
    tag = "comments"
)]
async fn get_comment_by_id(comment_id: web::Path<Uuid>, pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        get_comment(&mut conn, comment_id.into_inner())
    }).await;
    
    match result {
        Ok(comment_result) => handle_diesel_result(comment_result),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Server error".to_string())),
    }
}

#[utoipa::path(
    put,
    path = "/comments/{id}",
    request_body = Comment,
    responses(
        (status = 200, description = "Comment updated successfully", body = Comment),
        (status = 404, description = "Comment not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = Uuid, Path, description = "Comment ID")
    ),
    tag = "comments"
)]
async fn update_comment_handler(comment_id: web::Path<Uuid>, comment: web::Json<Comment>, pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        update_comment(&mut conn, comment_id.into_inner(), &comment.content)
    }).await;
    
    match result {
        Ok(comment_result) => handle_diesel_result(comment_result),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Server error".to_string())),
    }
}

#[utoipa::path(
    delete,
    path = "/comments/{id}",
    responses(
        (status = 200, description = "Comment deleted successfully"),
        (status = 404, description = "Comment not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = Uuid, Path, description = "Comment ID")
    ),
    tag = "comments"
)]
async fn delete_comment_handler(comment_id: web::Path<Uuid>, pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        delete_comment(&mut conn, comment_id.into_inner())
    }).await;
    
    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()>::success(())),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Server error".to_string())),
    }
}

#[utoipa::path(
    post,
    path = "/likes",
    request_body = Like,
    responses(
        (status = 200, description = "Like created successfully", body = Like),
        (status = 500, description = "Internal server error")
    ),
    tag = "likes"
)]
async fn create_like_handler(like: web::Json<Like>, pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        create_like(&mut conn, like.blog_id, like.user_id)
    }).await;
    
    match result {
        Ok(like_result) => handle_diesel_result(like_result),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Server error".to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/likes/{id}",
    responses(
        (status = 200, description = "Like found", body = Like),
        (status = 404, description = "Like not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Like ID")
    ),
    tag = "likes"
)]
async fn get_like_by_id(like_id: web::Path<Uuid>, pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        get_like(&mut conn, like_id.into_inner())
    }).await;
    
    match result {
        Ok(like_result) => handle_diesel_result(like_result),
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error("Server error".to_string())),
    }
}