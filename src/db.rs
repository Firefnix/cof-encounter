use rusqlite::{params, Connection, Row};

pub type SqlInteger = i32;

pub trait FromDb {
    fn from_db(conn: &Connection, row: &Row) -> Self;
}

#[derive(Debug)]
pub struct Illustration {
    pub width: Option<SqlInteger>,
    pub height: Option<SqlInteger>,
    pub path: String,
}

impl FromDb for Illustration {
    fn from_db(_conn: &Connection, row: &Row) -> Self {
        Illustration {
            width: row.get(1).unwrap(),
            height: row.get(2).unwrap(),
            path: row.get(3).unwrap(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Tag {
    pub name: String,
    pub desc: Option<String>,
}

impl FromDb for Tag {
    fn from_db(_conn: &Connection, row: &Row) -> Self {
        Tag {
            name: row.get(1).unwrap(),
            desc: row.get(2).unwrap(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Artifact {
    pub name: String,
    pub desc: Option<String>,
    pub price: Option<SqlInteger>,
    pub illustration: Option<Illustration>,
}

impl FromDb for Artifact {
    fn from_db(conn: &Connection, row: &Row) -> Self {
        Artifact {
            name: row.get(1).unwrap(),
            desc: row.get(2).unwrap(),
            price: row.get(3).unwrap(),
            illustration: get_by_id(conn, row.get(4).unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct Npc {
    pub name: String,
    pub force: Option<SqlInteger>,
    pub dex: Option<SqlInteger>,
    pub con: Option<SqlInteger>,
    pub int: Option<SqlInteger>,
    pub sag: Option<SqlInteger>,
    pub cha: Option<SqlInteger>,
    pub def: Option<SqlInteger>,
    pub pv: Option<SqlInteger>,
    pub init: Option<SqlInteger>,
    pub illustration: Option<Illustration>,
}

impl FromDb for Npc {
    fn from_db(conn: &Connection, row: &Row) -> Self {
        Npc {
            name: row.get(1).unwrap(),
            force: row.get(2).unwrap(),
            dex: row.get(3).unwrap(),
            con: row.get(4).unwrap(),
            int: row.get(5).unwrap(),
            sag: row.get(6).unwrap(),
            cha: row.get(7).unwrap(),
            def: row.get(8).unwrap(),
            pv: row.get(9).unwrap(),
            init: row.get(10).unwrap(),
            illustration: get_by_id(conn, row.get(11).unwrap()),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Place {
    pub name: String,
    pub desc: Option<String>,
    pub illustration: Option<Illustration>,
    pub tags: Vec<Tag>,
    pub npcs: Vec<Npc>,
    pub artifacts: Vec<Artifact>,
}

impl FromDb for Place {
    fn from_db(conn: &Connection, row: &Row) -> Self {
        Place {
            name: row.get(1).unwrap(),
            desc: row.get(2).unwrap(),
            illustration: get_by_id(conn, row.get(3).unwrap()),
            tags: get_items_from_id::<Tag, Place>(conn, row.get(0).unwrap()),
            npcs: get_items_from_id::<Npc, Place>(conn, row.get(0).unwrap()),
            artifacts: get_items_from_id::<Artifact, Place>(conn, row.get(0).unwrap()),
        }
    }
}

const DB_FILE: &str = "cof.db";

pub trait TableName {
    fn table_name() -> &'static str;
}

impl TableName for Illustration {
    fn table_name() -> &'static str {
        "illustration"
    }
}

impl TableName for Tag {
    fn table_name() -> &'static str {
        "tag"
    }
}

impl TableName for Artifact {
    fn table_name() -> &'static str {
        "object"
    }
}

impl TableName for Npc {
    fn table_name() -> &'static str {
        "npc"
    }
}

impl TableName for Place {
    fn table_name() -> &'static str {
        "place"
    }
}

fn is_db_file_present() -> bool {
    std::path::Path::new(DB_FILE).exists()
}

pub fn setup() -> Connection {
    let file_initialized = is_db_file_present();
    let conn = Connection::open(DB_FILE).expect("Could not open database file");
    if !file_initialized {
        initialize_db(&conn);
    }
    conn
}

fn initialize_db(conn: &Connection) {
    const INIT_SQL: &str = include_str!("../assets/init.sql");
    conn.execute_batch(INIT_SQL)
        .expect("Failed to initialize database");
    println!("Database initialized");
}

pub fn get_by_id<T: FromDb + TableName>(conn: &Connection, id: SqlInteger) -> Option<T> {
    let mut stmt = conn
        .prepare(&format!("SELECT * FROM {} WHERE id = ?1", T::table_name()))
        .unwrap();
    let iter = match stmt.query_map(params![id], |row| Ok(T::from_db(conn, row))) {
        Ok(iter) => Some(iter),
        Err(_) => None, // Return None if there is an error
    };
    match iter {
        Some(iter) => iter.filter_map(|item| item.ok()).next(),
        None => None,
    }
}

pub fn get_all<T: FromDb + TableName>(conn: &Connection) -> Vec<T> {
    let mut stmt = conn
        .prepare(&format!("SELECT * FROM {}", T::table_name()))
        .unwrap();
    let iter = match stmt.query_map([], |row| Ok(T::from_db(conn, row))) {
        Ok(iter) => iter,
        Err(_) => return vec![], // Return empty vector if there is an error
    };
    iter.filter_map(|item| item.ok()).collect()
}

fn get_items_from_id<A: FromDb + TableName, B: TableName>(
    conn: &Connection,
    id: SqlInteger,
) -> Vec<A> {
    // B = npc, A = place
    let table_name = format!("{}_{}s", B::table_name(), A::table_name());
    let mut stmt = conn
        .prepare(&format!(
            "SELECT * FROM {} WHERE {}_id = ?1",
            table_name,
            B::table_name()
        ))
        .expect(&format!("Table {} does not exist", table_name));
    let iter = match stmt.query_map(params![id], |row| Ok(A::from_db(conn, row))) {
        Ok(iter) => iter,
        Err(_) => return vec![], // Return empty vector if there is an error
    };
    iter.filter_map(|item| item.ok()).collect()
}
