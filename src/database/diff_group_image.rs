use crate::database::SortType;
use diesel::query_dsl::methods::OrderDsl;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};
use rocket::serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use validator::Validate;

#[derive(Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = crate::schema::diff_group_images)]
#[diesel(belongs_to(images, foreign_key = image_id))]
#[diesel(belongs_to(diff_groups, foreign_key = group_id))]
#[diesel(primary_key(image_id, group_id))]
#[serde(crate = "rocket::serde")]
pub struct DiffGroupImage {
    pub group_id: String,
    pub image_id: i32,
    pub sort_order: f64,
}

#[derive(Validate, Debug, Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::diff_group_images)]
#[serde(crate = "rocket::serde")]
pub struct NewDiffGroupImage {
    pub group_id: String,
    pub image_id: i32,
    #[validate(range(min = 0.0, message = "排序值必须大于等于0"))]
    pub sort_order: f64,
}

const NO_SORT_ORDER: f64 = -1.0;
const HEADER_SORT_ORDER: f64 = 0.0;
const FOOTER_SORT_ORDER: f64 = f64::MAX;
impl DiffGroupImage {}

impl NewDiffGroupImage {
    pub fn new(group_id: String, image_id: i32, sort: f64) -> Self {
        Self { group_id, image_id, sort_order: sort }
    }

    pub fn new_without_sort_order(group_id: String, image_id: i32) -> Self {
        Self { group_id, image_id, sort_order: NO_SORT_ORDER }
    }
}

impl PartialEq for &DiffGroupImage {
    fn eq(&self, other: &Self) -> bool {
        self.group_id == other.group_id
            && self.image_id == other.image_id
            && self.sort_order == other.sort_order
    }
}

impl DiffGroupImage {
    pub fn create(conn: &mut SqliteConnection, new_relation: &NewDiffGroupImage) -> QueryResult<Self> {
        use crate::schema::diff_group_images;
        let mut real_order = new_relation.sort_order;
        if new_relation.sort_order == NO_SORT_ORDER {
            real_order = match OrderDsl::order(diff_group_images::table
                                                   .filter(diff_group_images::group_id.eq(new_relation.group_id.clone())), diff_group_images::sort_order.desc())
                .first::<DiffGroupImage>(conn) {
                Ok(group_image) => group_image.sort_order + 1000.0,
                Err(_) => 1000.0,
            }
        }

        let wait_insert = NewDiffGroupImage {
            group_id: new_relation.group_id.clone(),
            image_id: new_relation.image_id,
            sort_order: real_order,
        };

        diesel::insert_into(diff_group_images::table)
            .values(&wait_insert)
            .execute(conn)?;
        diff_group_images::table
            .filter(diff_group_images::group_id.eq(&new_relation.group_id))
            .filter(diff_group_images::image_id.eq(&new_relation.image_id))
            .first(conn)
    }

    pub fn insert_between(conn: &mut SqliteConnection,
                          new_relation: &NewDiffGroupImage,
                          left: &DiffGroupImage,
                          right: &DiffGroupImage) -> QueryResult<Self> {
        use crate::schema::diff_group_images;
        let real_order = if right.sort_order == FOOTER_SORT_ORDER {
            Self::find_last_by_group_id(conn, new_relation.group_id.clone())?.sort_order + 1000.0
        } else {
            (left.sort_order + right.sort_order) / 2.0
        };

        let wait_insert = NewDiffGroupImage {
            group_id: new_relation.group_id.clone(),
            image_id: new_relation.image_id,
            sort_order: real_order,
        };

        diesel::insert_into(diff_group_images::table)
            .values(&wait_insert)
            .execute(conn)?;
        diff_group_images::table
            .filter(diff_group_images::group_id.eq(&new_relation.group_id))
            .filter(diff_group_images::image_id.eq(&new_relation.image_id))
            .first(conn)
    }

    pub fn list_by_group_id(conn: &mut SqliteConnection, group_id: String, sort_type: SortType) -> QueryResult<Vec<Self>> {
        use crate::schema::diff_group_images;
        let base_query = diff_group_images::table
            .filter(diff_group_images::group_id.eq(&group_id));

        match sort_type {
            SortType::Ascending => OrderDsl::order(base_query, diff_group_images::sort_order.asc()).load(conn),
            SortType::Descending => OrderDsl::order(base_query, diff_group_images::sort_order.desc()).load(conn),
        }
    }

    fn find_last_by_group_id(conn: &mut SqliteConnection, group_id: String) -> QueryResult<Self> {
        use crate::schema::diff_group_images;
        OrderDsl::order(diff_group_images::table
                            .filter(diff_group_images::group_id.eq(&group_id)), diff_group_images::sort_order.desc())
            .first(conn)
    }
}

