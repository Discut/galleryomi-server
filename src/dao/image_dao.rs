use rocket::http::Status;
use rocket::serde::json::Json;
use crate::bean::custom_error::CustomError;
use crate::database::init::MainDbConn;

// Rocket处理函数
/*#[post("/images", data = "<new_image>")]
pub async fn create_image(
    db: MainDbConn,
    new_image: Json<crate::database::image::NewImage>,
) -> Result<Json<crate::database::image::Image>, CustomError> {
    // 获取内部值并验证
    let image_data = new_image.into_inner();
    image_data.validate().map_err(|e| {
        CustomError::new(Status::BadRequest, format!("验证错误: {}", e))
    })?;
    new_image.validate().map_err(|e| {
        CustomError::new(Status::BadRequest, format!("验证错误: {}", e))
    })?;

    db.run(move |conn| {
        crate::database::image::Image::create(conn, &new_image.into_inner())
            .map(Json)
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => CustomError::new(Status::Conflict, "文件路径已存在"),
                _ => CustomError::from(e),
            })
    }).await
}

#[get("/images/<id>")]
pub async fn get_image(db: MainDbConn, id: i32) -> Result<Json<crate::database::image::Image>, CustomError> {
    db.run(move |conn| {
        crate::database::image::Image::find_by_id(conn, id)
            .map(Json)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    CustomError::new(Status::NotFound, "图片未找到")
                }
                _ => CustomError::from(e),
            })
    }).await
}*/