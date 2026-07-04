CREATE TABLE users (
    user_id TEXT PRIMARY KEY,
    tokens INTEGER DEFAULT 0,
    last_daily TEXT,
    last_job TEXT,
    debt INTEGER DEFAULT 0,
    in_stock_market INTEGER DEFAULT 1,
    xp INTEGER DEFAULT 0,
    level INTEGER DEFAULT 0,
    last_message_time TEXT,
    last_channel_id TEXT,
    message_count INTEGER DEFAULT 0
);

CREATE TABLE channels (
    channel_id TEXT PRIMARY KEY,
    owner_id TEXT,
    tax_rate INTEGER DEFAULT 0,
    in_stock_market INTEGER DEFAULT 0,
    stock_value REAL DEFAULT 100.0,
    stock_history TEXT DEFAULT '[]',
    stock_last_updated TEXT,
    FOREIGN KEY (owner_id) REFERENCES users(user_id)
);

CREATE TABLE items (
    item_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    price INTEGER NOT NULL,
    quantity INTEGER DEFAULT 0,
    owner_user_id TEXT,
    owner_channel_id TEXT,
    item_type TEXT,
    can_resell INTEGER DEFAULT 1,
    FOREIGN KEY (owner_user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (owner_channel_id) REFERENCES channels(channel_id) ON DELETE CASCADE
);

CREATE TABLE stock_shares (
    channel_id TEXT,
    user_id TEXT,
    shares REAL,
    purchase_price REAL,
    PRIMARY KEY (channel_id, user_id, purchase_price),
    FOREIGN KEY (channel_id) REFERENCES channels(channel_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE marriages (
    marriage_id TEXT PRIMARY KEY,
    spouse_1 TEXT,
    spouse_2 TEXT,
    joint_balance INTEGER DEFAULT 0,
    wedding_date TEXT,
    FOREIGN KEY (spouse_1) REFERENCES users(user_id),
    FOREIGN KEY (spouse_2) REFERENCES users(user_id)
);

CREATE TABLE proposals (
    proposal_id TEXT PRIMARY KEY,
    proposer_id TEXT,
    proposee_id TEXT,
    created_at TEXT,
    FOREIGN KEY (proposer_id) REFERENCES users(user_id),
    FOREIGN KEY (proposee_id) REFERENCES users(user_id)
);

CREATE TABLE guild_settings (
    guild_id TEXT PRIMARY KEY,
    log_channel_id TEXT
);

CREATE TABLE system_settings (
    key TEXT PRIMARY KEY,
    value TEXT
);

INSERT OR IGNORE INTO system_settings (key, value) VALUES ('central_bank', '0');
