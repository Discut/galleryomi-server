use crate::schema::tags::dsl::*;
use diesel::{ExpressionMethods, Insertable, JoinOnDsl, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper, SqliteConnection};
use rocket::serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(primary_key(tag_id))]
#[serde(crate = "rocket::serde")]
pub struct Tag {
    pub tag_id: i32,
    pub type_id: i32,
    pub tag_value: String,
}


#[derive(Validate, Debug, Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::tags)]
#[serde(crate = "rocket::serde")]
pub struct NewTag {
    pub tag_id: i32,
    #[validate(length(min = 2, message = "标签值不能为空，至少2个字符"))]
    pub tag_value: String,
}


// CRUD操作实现
impl Tag {
    pub fn create(
        conn: &mut SqliteConnection,
        new_tag: &NewTag,
    ) -> QueryResult<Self> {
        diesel::insert_into(tags)
            .values(new_tag)
            .execute(conn)?;

        tags.filter(tag_value.eq(&new_tag.tag_value))
            .filter(type_id.eq(&new_tag.tag_id))
            .first(conn)
    }

    pub fn find_by_id(conn: &mut SqliteConnection, id: i32) -> QueryResult<Self> {
        tags.filter(tag_id.eq(id)).first(conn)
    }

    pub fn find_by_name(conn: &mut SqliteConnection, name: &str) -> QueryResult<Self> {
        tags.filter(tag_value.eq(name)).first(conn)
    }

    pub fn update(conn: &mut SqliteConnection, id: i32, new_tag: &NewTag) -> QueryResult<Self> {
        diesel::update(tags.filter(tag_id.eq(id)))
            .set(new_tag)
            .execute(conn)?;

        Self::find_by_id(conn, id)
    }

    pub fn delete(conn: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(tags.filter(tag_id.eq(id))).execute(conn)
    }

    pub fn list_by_image_id(conn: &mut SqliteConnection, id_from_image: i32) -> QueryResult<Vec<Self>> {
        use crate::schema::{image_tags, tags};

        let query = tags.inner_join(image_tags::table.on(tags::tag_id.eq(image_tags::tag_id)))
            .filter(image_tags::image_id.eq(id_from_image));
        query.select(Tag::as_select()).load(conn)
    }

    pub fn collect_by_image_id(conn: &mut SqliteConnection,
                               id_from_image: i32,
                               page: i64,
                               per_page: i64, ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema::{image_tags, tags};

        let query = tags.inner_join(image_tags::table.on(tags::tag_id.eq(image_tags::tag_id)))
            .filter(image_tags::image_id.eq(id_from_image));

        let total = query.clone().count().get_result(conn)?;

        let record = query
            .select(Tag::as_select())
            .offset((page - 1) * per_page)
            .limit(per_page)
            .load(conn)?;

        Ok((record, total))
    }
}