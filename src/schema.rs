// @generated automatically by Diesel CLI.

diesel::table! {
    article (id) {
        id -> Integer,
        title -> Text,
        author -> Text,
        content -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    diff_group_images (group_id, image_id) {
        group_id -> Text,
        image_id -> Integer,
        sort_order -> Double,
    }
}

diesel::table! {
    diff_groups (group_id) {
        group_id -> Text,
        group_name -> Text,
        cover_image_id -> Nullable<Integer>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    image_tags (image_id, tag_id) {
        image_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    images (id) {
        id -> Integer,
        file_path -> Text,
        collection_path -> Text,
        filesize -> BigInt,
        checksum -> Nullable<Text>,
        exif_json -> Nullable<Text>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

diesel::table! {
    tag_types (type_id) {
        type_id -> Integer,
        type_name -> Text,
    }
}

diesel::table! {
    tags (tag_id) {

        tag_id -> Integer,
        type_id -> Integer,
        tag_value -> Text,
    }
}

diesel::joinable!(diff_group_images -> diff_groups (group_id));
diesel::joinable!(diff_group_images -> images (image_id));
diesel::joinable!(diff_groups -> images (cover_image_id));
diesel::joinable!(image_tags -> images (image_id));
diesel::joinable!(image_tags -> tags (tag_id));
diesel::joinable!(tags -> tag_types (type_id));

diesel::allow_tables_to_appear_in_same_query!(
    article,
    diff_group_images,
    diff_groups,
    image_tags,
    images,
    tag_types,
    tags,
);
