-- 添加认证性能优化索引
-- 为 settings 表的 key 列添加唯一索引（加快密码查询）
CREATE UNIQUE INDEX IF NOT EXISTS idx_settings_key ON settings(key);

-- 为 events 表的 id 列显式索引（虽然是 PK 但显式声明更清楚）
CREATE INDEX IF NOT EXISTS idx_events_id ON events(id) WHERE id IS NOT NULL;
