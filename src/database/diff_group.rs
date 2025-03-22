use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection, TextExpressionMethods};
use rocket::serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = crate::schema::diff_groups)]
#[diesel(primary_key(group_id))]
#[diesel(belongs_to(images, foreign_key = cover_image_id))]
#[serde(crate = "rocket::serde")]
pub struct DiffGroup {
    pub group_id: String,
    pub group_name: String,
    pub cover_image_id: Option<i32>,
    pub created_at: String,
}

#[derive(Validate, Debug, Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::diff_groups)]
#[serde(crate = "rocket::serde")]
pub struct NewDiffGroup {
    #[validate(length(min = 36, message = "差分组uuid长度错误"))]
    pub group_id: String,

    #[validate(length(min = 2, message = "差分组名称需大于等于2个字符"))]
    pub group_name: String,
    pub cover_image_id: Option<i32>,
}


// CRUD操作实现
impl DiffGroup {
    pub fn create(
        conn: &mut SqliteConnection,
        new_group: &NewDiffGroup,
    ) -> QueryResult<Self> {
        use crate::schema::diff_groups;
        diesel::insert_into(diff_groups::table)
            .values(new_group)
            .execute(conn)?;
        diff_groups::table
            .order(diff_groups::created_at.desc())
            .first(conn)
    }

    pub fn find_by_id(conn: &mut SqliteConnection, id: String) -> QueryResult<Self> {
        use crate::schema::diff_groups;
        diff_groups::table
            .filter(diff_groups::group_id.eq(id))
            .first(conn)
    }

    pub fn find_by_name(conn: &mut SqliteConnection, name: String) -> QueryResult<Self> {
        use crate::schema::diff_groups::dsl::{diff_groups, group_name};

        diff_groups
            .filter(group_name.eq(name))
            .first(conn)
    }

    pub fn find_by_keyword(conn: &mut SqliteConnection, keyword: String) -> QueryResult<Vec<Self>> {
        use crate::schema::diff_groups::dsl::{diff_groups, group_name};
        diff_groups
            .filter(group_name.like(format!("%{}%", keyword)))
            .load(conn)
    }


    pub fn update(conn: &mut SqliteConnection, new_group: &NewDiffGroup) -> QueryResult<Self> {
        use crate::schema::diff_groups;
        let id = new_group.group_id.clone();
        diesel::update(diff_groups::table.filter(diff_groups::group_id.eq(&id)))
            .set(new_group)
            .execute(conn)?;
        Self::find_by_id(conn, id)
    }
}

impl NewDiffGroup {
    pub fn new_with_id(group_id: String, group_name: String, cover_image_id: Option<i32>) -> Self {
        Self {
            group_id,
            group_name,
            cover_image_id,
        }
    }

    pub fn new(group_name: String, cover_image_id: Option<i32>) -> Self {
        let group_id = uuid::Uuid::new_v4().to_string();
        Self {
            group_id,
            group_name,
            cover_image_id,
        }
    }
}