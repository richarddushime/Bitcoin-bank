CREATE TABLE IF NOT EXISTS users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT, 
    full_name TEXT,
    username TEXT UNIQUE,
    email TEXT, 
    password TEXT,
    api_key TEXT
);

CREATE TABLE IF NOT EXISTS users_account_details (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER, 
    balance INTEGER,
    account_address TEXT UNIQUE,
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS users_spend_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER, 
    source_address TEXT UNIQUE,
    destination_address TEXT UNIQUE,
    amount_spent INTEGER,
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS users_secret (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER, 
    secret_key TEXT UNIQUE,
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);