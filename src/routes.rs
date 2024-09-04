use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

use crate::models::{User, Blog, Comment, Like};
use crate::orm::{create_user, create_blog, create_comment, create_like, get_user, get_blog, update_blog, delete_blog};
use crate::db::DbPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/users", web::post().to(create_user_handler))
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/blogs", web::post().to(create_blog_handler))
            .route("/blogs/{id}", web::get().to(get_blog_by_id))
            .route("/blogs/{id}", web::put().to(update_blog_by_id))
            .route("/blogs/{id}", web::delete().to(delete_blog_by_id))
            .route("/comments", web::post().to(add_comment))
            .route("/likes", web::post().to(add_like))
    );
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
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        create_user(&conn, &user.username, &user.email, &user.password_hash)
    }).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create user"})),
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
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        get_user(&conn, user_id.into_inner())
    }).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "User not found"})),
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
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        create_blog(&conn, &blog.title, &blog.content, blog.author_id)
    }).await;
    match result {
        Ok(blog) => HttpResponse::Ok().json(blog),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create blog"})),
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
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        get_blog(&conn, blog_id.into_inner())
    }).await;
    match result {
        Ok(blog) => HttpResponse::Ok().json(blog),
        Err(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Blog not found"})),
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
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        update_blog(&conn, blog_id.into_inner(), &blog.title, &blog.content)
    }).await;
    match result {
        Ok(blog) => HttpResponse::Ok().json(blog),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update blog"})),
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
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        delete_blog(&conn, blog_id.into_inner())
    }).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "Blog deleted successfully"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to delete blog"})),
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
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        create_comment(&conn, comment.blog_id, comment.user_id, &comment.content, comment.parent_comment_id)
    }).await;
    match result {
        Ok(comment) => HttpResponse::Ok().json(comment),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create comment"})),
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
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        get_comment(&conn, comment_id.into_inner())
    }).await;
    match result {
        Ok(comment) => HttpResponse::Ok().json(comment),
        Err(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Comment not found"})),
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
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        update_comment(&conn, comment_id.into_inner(), &comment.content)
    }).await;
    match result {
        Ok(comment) => HttpResponse::Ok().json(comment),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update comment"})),
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
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        delete_comment(&conn, comment_id.into_inner())
    }).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "Comment deleted successfully"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to delete comment"})),
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
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        create_like(&conn, like.blog_id, like.user_id)
    }).await;
    match result {
        Ok(like) => HttpResponse::Ok().json(like),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create like"})),
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
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || {
        get_like(&conn, like_id.into_inner())
    }).await;
    match result {
        Ok(like) => HttpResponse::Ok().json(like),
        Err(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Like not found"})),
    }
}
