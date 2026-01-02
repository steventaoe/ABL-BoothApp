-- migrations/202601020001_init_schema.sql

-- 1. 系统设置表 (用于存储管理员密码Hash、JWT密钥等全局配置)
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL
);

-- 2. 全局商品库 (Master Products)
CREATE TABLE IF NOT EXISTS master_products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    product_code TEXT NOT NULL UNIQUE, -- 商品条码/编号
    name TEXT NOT NULL,
    default_price REAL NOT NULL,       -- 默认单价
    image_url TEXT,                    -- 图片存储路径 (相对路径)
    category TEXT,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. 漫展场次 (Events)
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    event_date TEXT NOT NULL,          -- 格式 YYYY-MM-DD
    location TEXT,
    status TEXT NOT NULL DEFAULT '未进行', -- 枚举: '未进行', '进行中', '已结束'
    vendor_password TEXT,              -- 该场次摊主的独立密码 (Hash)
    payment_qr_code_path TEXT,         -- 收款码路径
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 4. 场次库存商品 (Event Products)
-- 这是一个关联表，连接 Events 和 MasterProducts，并记录当前库存
CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER NOT NULL,
    master_product_id INTEGER NOT NULL,
    
    product_code TEXT NOT NULL,        -- 冗余字段，方便查询
    name TEXT NOT NULL,                -- 冗余字段，也就是 Snapshot (快照)，防止原商品改名后订单混乱
    
    price REAL NOT NULL,               -- 该场次的实际售价
    initial_stock INTEGER NOT NULL,    -- 初始库存
    current_stock INTEGER NOT NULL,    -- 当前剩余库存
    
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE,
    FOREIGN KEY (master_product_id) REFERENCES master_products(id)
);

-- 索引，加速库存查询
CREATE INDEX idx_products_event_id ON products(event_id);

-- 5. 订单主表 (Orders)
CREATE TABLE IF NOT EXISTS orders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER NOT NULL,
    total_amount REAL NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'completed', 'cancelled'
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);

-- 6. 订单明细表 (Order Items)
CREATE TABLE IF NOT EXISTS order_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,       -- 指向 products 表 (库存表) 的 ID
    
    -- 快照数据：即使库存表或总表被删改，订单记录应保持历史原样
    product_name TEXT NOT NULL,
    product_price REAL NOT NULL,
    quantity INTEGER NOT NULL,
    
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE
);