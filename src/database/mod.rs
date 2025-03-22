pub mod init;
pub mod image;
mod tag_type;
mod tag;
mod image_tag;
mod diff_group;
mod diff_group_image;

pub enum SortType {
    Ascending,
    Descending
}