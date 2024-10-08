use diesel::prelude::*;
use diesel::pg::PgConnection;
use uuid::Uuid;
use crate::models::{User, Blog, Comment, Like};
use crate::schema::{users, blogs, comments, likes};
// use crate::orm::{ update_comment, delete_comment, get_like};

#[allow(dead_code)]
pub fn create_user(conn: &mut PgConnection, username: &str, email: &str, password_hash: &str) -> Result<User, diesel::result::Error> {
    let new_user = NewUser {
        username,
        email,
        password_hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

#[allow(dead_code)]
pub fn get_user(conn: &mut PgConnection, user_id: Uuid) -> Result<User, diesel::result::Error> {
    users::table.find(user_id).get_result::<User>(conn)
}

#[allow(dead_code)]
pub fn update_user(conn: &mut PgConnection, user_id: Uuid, username: &str, email: &str) -> Result<User, diesel::result::Error> {
    diesel::update(users::table.find(user_id))
        .set((
            users::username.eq(username),
            users::email.eq(email),
        ))
        .get_result::<User>(conn)
}


#[allow(dead_code)]
pub fn delete_user(conn: &mut PgConnection, user_id: Uuid) -> Result<usize, diesel::result::Error> {
    diesel::delete(users::table.find(user_id))
        .execute(conn)
}


#[allow(dead_code)]
pub fn create_blog(conn: &mut PgConnection, title: &str, content: &str, author_id: Uuid) -> Result<Blog, diesel::result::Error> {
    let new_blog = NewBlog {
        title,
        content,
        author_id,
    };

    diesel::insert_into(blogs::table)
        .values(&new_blog)
        .get_result(conn)
}

#[allow(dead_code)]
pub fn get_blog(conn: &mut PgConnection, blog_id: Uuid) -> Result<Blog, diesel::result::Error> {
    blogs::table.find(blog_id).get_result::<Blog>(conn)
}

#[allow(dead_code)]
pub fn update_blog(conn: &mut PgConnection, blog_id: Uuid, title: &str, content: &str) -> Result<Blog, diesel::result::Error> {
    diesel::update(blogs::table.find(blog_id))
        .set((
            blogs::title.eq(title),
            blogs::content.eq(content),
            blogs::updated_at.eq(diesel::dsl::now),
        ))
        .get_result::<Blog>(conn)
}


#[allow(dead_code)]
pub fn delete_blog(conn: &mut PgConnection, blog_id: Uuid) -> Result<usize, diesel::result::Error> {
    diesel::delete(blogs::table.find(blog_id))
        .execute(conn)
}


#[allow(dead_code)]
pub fn create_comment(conn: &mut PgConnection, blog_id: Uuid, user_id: Uuid, content: &str, parent_comment_id: Option<Uuid>) -> Result<Comment, diesel::result::Error> {
    let new_comment = NewComment {
        blog_id,
        user_id,
        content,
        parent_comment_id,
    };

    diesel::insert_into(comments::table)
        .values(&new_comment)
        .get_result(conn)
}


#[allow(dead_code)]
pub fn get_comment(conn: &mut PgConnection, comment_id: Uuid) -> Result<Comment, diesel::result::Error> {
    comments::table.find(comment_id).get_result::<Comment>(conn)
}


#[allow(dead_code)]
pub fn update_comment(conn: &mut PgConnection, comment_id: Uuid, content: &str) -> Result<Comment, diesel::result::Error> {
    diesel::update(comments::table.find(comment_id))
        .set(comments::content.eq(content))
        .get_result::<Comment>(conn)
}

#[allow(dead_code)]
pub fn delete_comment(conn: &mut PgConnection, comment_id: Uuid) -> Result<usize, diesel::result::Error> {
    diesel::delete(comments::table.find(comment_id))
        .execute(conn)
}

#[allow(dead_code)]
pub fn create_like(conn: &mut PgConnection, blog_id: Uuid, user_id: Uuid) -> Result<Like, diesel::result::Error> {
    let new_like = NewLike {
        blog_id,
        user_id,
    };

    diesel::insert_into(likes::table)
        .values(&new_like)
        .get_result(conn)
}

#[allow(dead_code)]
pub fn get_like(conn: &mut PgConnection, like_id: Uuid) -> Result<Like, diesel::result::Error> {
    likes::table.find(like_id).get_result::<Like>(conn)
}


#[allow(dead_code)]
pub fn delete_like(conn: &mut PgConnection, like_id: Uuid) -> Result<usize, diesel::result::Error> {
    diesel::delete(likes::table.find(like_id))
        .execute(conn)
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
    username: &'a str,
    email: &'a str,
    password_hash: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = blogs)]
struct NewBlog<'a> {
    title: &'a str,
    content: &'a str,
    author_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
struct NewComment<'a> {
    blog_id: Uuid,
    user_id: Uuid,
    content: &'a str,
    parent_comment_id: Option<Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = likes)]
struct NewLike {
    blog_id: Uuid,
    user_id: Uuid,
}