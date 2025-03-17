use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use rocket::serde::Serialize;

#[derive(Debug, Queryable, Serialize, Identifiable)]
#[diesel(table_name = crate::schema::tag_types)]
#[diesel(primary_key(type_id))]
#[serde(crate = "rocket::serde")]
pub struct TagType {
    pub type_id: i32,
    pub type_name: String,
}

impl TagType {
    ///
    /// 查找指定ID的TagType
    ///
    pub fn find_by_id(conn: &mut diesel::SqliteConnection, id: i32) -> QueryResult<Self> {
        use crate::schema::tag_types::dsl::*;
        tag_types.filter(type_id.eq(id)).first(conn)
        // QueryDsl::filter(tag_types, type_id.eq(id)).first(conn)
    }

    pub fn find_by_name(conn: &mut diesel::SqliteConnection, name: &str) -> QueryResult<Self> {
        use crate::schema::tag_types::dsl::*;
        tag_types.filter(type_name.eq(name)).first(conn)
    }

    pub fn find_all(conn: &mut diesel::SqliteConnection) -> QueryResult<Vec<Self>> {
        use crate::schema::tag_types::dsl::*;
        tag_types.load(conn)
    }
}