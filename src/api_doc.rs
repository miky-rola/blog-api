use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::create_user_handler,
        crate::routes::get_user_by_id,
        crate::routes::create_blog_handler,
        crate::routes::get_blog_by_id,
        crate::routes::update_blog_by_id,
        crate::routes::delete_blog_by_id,
        crate::routes::create_comment_handler,
        crate::routes::get_comment_by_id,
        crate::routes::update_comment_handler,
        crate::routes::delete_comment_handler,
        crate::routes::create_like_handler,
        crate::routes::get_like_by_id
    ),
    components(
        schemas(crate::models::User, crate::models::Blog, crate::models::Comment, crate::models::Like)
    ),
    tags(
        (name = "users", description = "User management API"),
        (name = "blogs", description = "Blog management API"),
        (name = "comments", description = "Comment management API"),
        (name = "likes", description = "Like management API")
    )
)]
pub struct ApiDoc;