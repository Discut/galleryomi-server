mod bean;
mod schema;
mod routes;
mod database;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use routes::test_routes::*;
use database::init::MainDbConn;

#[launch]
fn rocket() -> _ {
    rocket::build()
        // database
        .attach(MainDbConn::fairing())
        .mount("/", routes![index])
        // add api
        .mount("/", routes![get_all_articles, get_article_by_id])
        .mount("/", routes![post_article, put_article])
        .mount("/", routes![delete_all_articles, delete_article])
}
