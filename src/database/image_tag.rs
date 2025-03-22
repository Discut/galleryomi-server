use crate::schema::image_tags::dsl::image_tags;
use crate::schema::image_tags::image_id;
use crate::schema::images::dsl::images;
use crate::schema::tags::dsl::tags;
use diesel::{ExpressionMethods, Insertable, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};
use rocket::serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable, Associations)]
#[diesel(belongs_to(images, foreign_key = image_id))]
#[diesel(belongs_to(tags, foreign_key = tag_id))]
#[diesel(table_name = crate::schema::image_tags)]
#[diesel(primary_key(image_id, tag_id))]
pub struct ImageTag {
    pub image_id: i32,
    pub tag_id: i32,
}


#[derive(Validate, Debug, Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::image_tags)]
#[serde(crate = "rocket::serde")]
pub struct NewImageTag {
    pub image_id: i32,
    pub tag_id: i32,
}

impl ImageTag {
    pub fn create(conn: &mut SqliteConnection, new_relation: &NewImageTag) -> QueryResult<Self> {
        diesel::insert_into(image_tags)
            .values(new_relation)
            .execute(conn)?;
        image_tags.order(image_id.desc()).first(conn)
    }
}