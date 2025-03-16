use crate::schema::article;
use diesel::Insertable;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "article"]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub content: String,
    pub created_at: String,
}
#[derive(Debug, Serialize, Deserialize, Queryable, Clone, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "article"]
pub struct PostArticle {
    pub title: String,
    pub author: String,
    pub content: String,
    pub created_at: String,
}