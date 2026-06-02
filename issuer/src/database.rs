use rusqlite::{params, Connection, Result};

#[allow(dead_code)]
pub struct UserRecord {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: u64,
}

#[allow(dead_code)]
pub struct ActivationCodeRecord {
    pub code: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: u64,
    pub used: bool,
}

pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    
    // Create users table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            email TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            date_of_birth INTEGER NOT NULL
        )",
        [],
    )?;

    // Create activation_codes table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS activation_codes (
            code TEXT PRIMARY KEY,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            date_of_birth INTEGER NOT NULL,
            used INTEGER DEFAULT 0
        )",
        [],
    )?;

    Ok(conn)
}

pub fn create_activation_code(
    conn: &Connection,
    code: &str,
    first_name: &str,
    last_name: &str,
    dob: u64,
) -> Result<()> {
    conn.execute(
        "INSERT INTO activation_codes (code, first_name, last_name, date_of_birth, used)
         VALUES (?1, ?2, ?3, ?4, 0)",
        params![code, first_name, last_name, dob],
    )?;
    Ok(())
}

pub fn get_activation_code(conn: &Connection, code: &str) -> Result<Option<ActivationCodeRecord>> {
    let mut stmt = conn.prepare(
        "SELECT code, first_name, last_name, date_of_birth, used FROM activation_codes WHERE code = ?1"
    )?;
    
    let mut rows = stmt.query_map(params![code], |row| {
        let used_int: i32 = row.get(4)?;
        Ok(ActivationCodeRecord {
            code: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            date_of_birth: row.get(3)?,
            used: used_int != 0,
        })
    })?;

    if let Some(res) = rows.next() {
        Ok(Some(res?))
    } else {
        Ok(None)
    }
}

pub fn mark_activation_code_used(conn: &Connection, code: &str) -> Result<()> {
    conn.execute(
        "UPDATE activation_codes SET used = 1 WHERE code = ?1",
        params![code],
    )?;
    Ok(())
}

pub fn create_user(
    conn: &Connection,
    username: &str,
    email: &str,
    password_hash: &str,
    first_name: &str,
    last_name: &str,
    dob: u64,
) -> Result<()> {
    conn.execute(
        "INSERT INTO users (username, email, password_hash, first_name, last_name, date_of_birth)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![username, email, password_hash, first_name, last_name, dob],
    )?;
    Ok(())
}

pub fn get_user_by_username(conn: &Connection, username: &str) -> Result<Option<UserRecord>> {
    let mut stmt = conn.prepare(
        "SELECT username, email, password_hash, first_name, last_name, date_of_birth FROM users WHERE username = ?1"
    )?;

    let mut rows = stmt.query_map(params![username], |row| {
        Ok(UserRecord {
            username: row.get(0)?,
            email: row.get(1)?,
            password_hash: row.get(2)?,
            first_name: row.get(3)?,
            last_name: row.get(4)?,
            date_of_birth: row.get(5)?,
        })
    })?;

    if let Some(res) = rows.next() {
        Ok(Some(res?))
    } else {
        Ok(None)
    }
}
