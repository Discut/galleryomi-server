use crate::schema::images::dsl::images;
use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;
use diesel::query_dsl::methods::OrderDsl;
use diesel::prelude::*; // 包含 FilterDsl 和 QueryDsl
use diesel::{ExpressionMethods, RunQueryDsl, SqliteConnection};
use rocket::serde::{Deserialize, Serialize};
use validator::Validate;

use regex::Regex;
use std::sync::LazyLock;

static RE_PATH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^/(?:[^/]+/)*[^/]+$").unwrap()
});

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[diesel(table_name = crate::schema::images)]
#[diesel(primary_key(id))]
#[serde(crate = "rocket::serde")]
pub struct Image {
    pub id: i32,
    #[serde(rename = "filePath")]
    pub file_path: String,
    #[serde(rename = "collectionPath")]
    pub collection_path: String,
    pub filesize: i64,
    pub checksum: Option<String>,
    #[serde(rename = "exifJson")]
    pub exif_json: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
}

#[derive(Validate, Debug, Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::images)]
#[serde(crate = "rocket::serde")]
pub struct NewImage {
    #[validate(length(min = 1, message = "文件路径不能为空"))]
    #[serde(rename = "filePath")]
    pub file_path: String,

    #[validate(regex(
        path = RE_PATH ,
        message = "合集路径必须为有效的Unix风格路径"
    ))]
    #[serde(rename = "collectionPath")]
    pub collection_path: String,

    #[validate(range(min = 1, message = "文件大小必须大于0"))]
    pub filesize: i64,

    #[validate(length(equal = 64, message = "校验和必须为64字符"))]
    pub checksum: Option<String>,

    #[serde(rename = "exifJson")]
    pub exif_json: Option<String>,
}

// CRUD操作实现
impl Image {
    pub fn create(
        conn: &mut SqliteConnection,
        new_image: &NewImage,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::images::dsl::*;

        diesel::insert_into(images)
            .values(new_image)
            .execute(conn)?;

        OrderDsl::order(images, id.desc())
            .first(conn)
    }

    pub fn find_by_id(
        conn: &mut SqliteConnection,
        image_id: i32,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::images::dsl::*;

        FilterDsl::filter(images, id.eq(image_id))
            .first(conn)
    }

    pub fn find_by_path(
        conn: &mut SqliteConnection,
        path: &str,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::images::dsl::*;

        images.filter(file_path.eq(path))
            .first(conn)
    }

    pub fn update(
        conn: &mut SqliteConnection,
        image_id: i32,
        update_data: &NewImage,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::images::dsl::*;

        diesel::update(FilterDsl::filter(images, id.eq(image_id)))
            .set(update_data)
            .execute(conn)?;

        Self::find_by_id(conn, image_id)
    }

    pub fn delete(
        conn: &mut SqliteConnection,
        image_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::images::dsl::*;

        diesel::delete(FilterDsl::filter(images, id.eq(image_id)))
            .execute(conn)
    }

    pub fn list_by_collection(
        conn: &mut SqliteConnection,
        collection: &str,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<Self>, i64), diesel::result::Error> {
        use crate::schema::images::dsl::*;

        let query = OrderDsl::order(FilterDsl::filter(images, collection_path.eq(collection)), created_at.desc());

        // 分页
        let total = query.count().get_result(conn)?;
        let records = query
            .offset((page - 1) * per_page)
            .limit(per_page)
            .load(conn)?;

        Ok((records, total))
    }
}