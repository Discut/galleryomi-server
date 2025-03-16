
use validator::Validate;

/*#[cfg(test)]
mod tests {


    #[test]
    fn test_valid_image() {
        let img = NewImage {
            file_path: "/valid/path.jpg".into(),
            collection_path: "/valid/collection".into(),
            filesize: 1024,
            checksum: Some("a".repeat(64)),
            exif_json: None,
        };
        assert!(img.validate().is_ok());
    }

    #[test]
    fn test_invalid_path() {
        let img = NewImage {
            file_path: "".into(), // 空路径
            collection_path: "invalid_path".into(), // 缺少前导斜杠
            filesize: 0, // 无效大小
            checksum: Some("short".into()),
            exif_json: None,
        };
        let errors = img.validate().unwrap_err();
        assert_eq!(errors.field_errors().len(), 3);
    }
}*/