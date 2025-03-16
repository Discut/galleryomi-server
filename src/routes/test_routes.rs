use diesel::{prelude::*, QueryDsl, RunQueryDsl};
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{delete, get, post, put, response::status::Created, response::Debug};

use crate::bean::db::{Article, PostArticle};
use crate::schema::article;
use crate::MainDbConn;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/")]
pub fn index() -> Value {
    json!({"kkbt":"Hello, world!"})
}

// 查 all
#[get("/article")]
pub async fn get_all_articles(db: MainDbConn) -> Result<Json<Vec<Article>>> {
    let all = db
        .run(move |conn| article::table.load::<Article>(conn))
        .await?;

    Ok(Json(all))
}
// 查 by id
#[get("/article/<in_id>")]
pub async fn get_article_by_id(db: MainDbConn, in_id: i32) -> Option<Json<Article>> {
    db.run(move |conn| article::table.filter(article::id.eq(in_id)).first(conn))
        .await
        .map(Json)
        .ok()
}
// 增
#[post("/article", format = "json", data = "<in_article>")]
pub async fn post_article(
    db: MainDbConn,
    in_article: Json<PostArticle>,
) -> Result<Created<Json<PostArticle>>> {
    let article_in = in_article.clone();
    db.run(move |conn| {
        diesel::insert_into(article::table)
            .values(&*article_in)
            .execute(conn)
    })
        .await?;
    Ok(Created::new("/").body(in_article))
}

// 改 by id
#[put("/article/<in_id>", format = "json", data = "<in_article>")]
pub async fn put_article(
    db: MainDbConn,
    in_id: i32,
    in_article: Json<PostArticle>,
) -> Result<Option<()>> {
    let affected = db
        .run(move |conn| {
            diesel::update(article::table.filter(article::id.eq(in_id)))
                .set(in_article.into_inner())
                .execute(conn)
        })
        .await?;
    Ok((affected == 1).then(|| ()))
}

// 删 by id
#[delete("/article/<in_id>")]
pub async fn delete_article(db: MainDbConn, in_id: i32) -> Result<Option<()>> {
    let affected = db
        .run(move |conn| {
            diesel::delete(article::table)
                .filter(article::id.eq(&in_id))
                .execute(conn)
        })
        .await?;
    Ok((affected == 1).then(|| ()))
}

// 删 all
#[delete("/article/all")]
pub async fn delete_all_articles(db: MainDbConn) -> Result<()> {
    db.run(move |conn| diesel::delete(article::table).execute(conn))
        .await?;

    Ok(())
}