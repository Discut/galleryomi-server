-- Your SQL goes here
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
-- 提升并发写入性能

-- 图片元数据表（核心实体）
CREATE TABLE IF NOT EXISTS images
(
    id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    file_path       TEXT    NOT NULL UNIQUE CHECK (length(file_path) > 0), -- 文件绝对路径
    collection_path TEXT    NOT NULL CHECK (collection_path LIKE '/%'),    -- 合集路径（Linux风格）
    filesize        INTEGER NOT NULL CHECK (filesize > 0),                 -- 文件大小（字节）
    checksum        TEXT CHECK (length(checksum) = 64),                    -- SHA256校验（可选）
    exif_json       TEXT,                                                  -- EXIF元数据（JSON格式）
    created_at      DATETIME NOT NULL DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')),
    modified_at     DATETIME NOT NULL DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW'))
);

-- 标签类型字典表（预置作者/来源/其他分类）
CREATE TABLE IF NOT EXISTS tag_types
(
    type_id   INTEGER PRIMARY KEY CHECK (type_id BETWEEN 1 AND 3),
    type_name TEXT NOT NULL UNIQUE
);
INSERT OR IGNORE INTO tag_types
VALUES (1, 'author'),
       (2, 'source'),
       (3, 'other');

-- 标签表（带类型约束）
CREATE TABLE IF NOT EXISTS tags
(
    tag_id    INTEGER PRIMARY KEY AUTOINCREMENT,
    type_id   INTEGER NOT NULL REFERENCES tag_types (type_id),
    tag_value TEXT    NOT NULL CHECK (length(tag_value) >= 2),
    UNIQUE (type_id, tag_value) -- 防止重复标签
);

-- 图片-标签关联表（多对多关系）
CREATE TABLE IF NOT EXISTS image_tags
(
    image_id INTEGER NOT NULL REFERENCES images (id) ON DELETE CASCADE,
    tag_id   INTEGER NOT NULL REFERENCES tags (tag_id) ON DELETE CASCADE,
    PRIMARY KEY (image_id, tag_id)
);

-- 差分组表（逻辑分组）
CREATE TABLE IF NOT EXISTS diff_groups
(
    group_id       TEXT PRIMARY KEY CHECK (length(group_id) = 36),    -- UUIDv4
    group_name     TEXT    NOT NULL CHECK (length(group_name) >= 2),
    cover_image_id INTEGER REFERENCES images (id) ON DELETE SET NULL, -- 可空封面
    created_at     DATETIME DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW'))
);

-- 差分组-图片关联表（带排序功能）
CREATE TABLE IF NOT EXISTS diff_group_images
(
    group_id   TEXT    NOT NULL REFERENCES diff_groups (group_id) ON DELETE CASCADE,
    image_id   INTEGER NOT NULL REFERENCES images (id) ON DELETE CASCADE,
    sort_order INTEGER NOT NULL CHECK (sort_order >= 0),
    PRIMARY KEY (group_id, image_id)
);

-- 索引优化（根据查询模式设计）
CREATE INDEX IF NOT EXISTS idx_image_path ON images (file_path);
CREATE INDEX IF NOT EXISTS idx_collection ON images (collection_path);
CREATE INDEX IF NOT EXISTS idx_tag_search ON tags (tag_value);
CREATE INDEX IF NOT EXISTS idx_diff_group_name ON diff_groups (group_name);
CREATE INDEX IF NOT EXISTS idx_diff_group_order ON diff_group_images (sort_order);

-- 更新时间戳触发器
CREATE TRIGGER IF NOT EXISTS update_image_timestamp
    AFTER UPDATE
    ON images
    FOR EACH ROW
BEGIN
    UPDATE images
    SET modified_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')
    WHERE id = OLD.id;
END;