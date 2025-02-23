use std::io;
use rusqlite::Connection;

use crate::db;
use crate::display;

pub fn list(conn: &Connection) {
    println!("Sélectionnez un type d'élément à lister :\n[1] Lieux\n[2] PNJs\n[3] Artefacts\n");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    match choice.trim() {
        "1" => list_all::<db::Place>(conn),
        "2" => list_all::<db::Npc>(conn),
        "3" => list_all::<db::Artifact>(conn),
        _ => println!("Choix invalide"),
    };
}

fn list_all<T: db::FromDb + db::TableName + ToString + display::Titled>(conn: &Connection) {
    let items: Vec<T> = db::get_all(conn);
    for item in items {
        display::display_in_text_box(&item);
        println!();
    }
}
