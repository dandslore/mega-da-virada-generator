use std::path::Path;
use rusqlite::{params, Connection, OptionalExtension};
use crate::{database};

pub fn bootstrap_mega_sena_data_from_csv() -> anyhow::Result<()> {

    let db_path = "mega_sena.db";
    let csv_mega_sena_path = "mega_sena.csv";

    let mut conn = Connection::open(db_path)?;
    println!("Conectado ao SQLite em {}", db_path);


    if !mega_sena_table_has_data(&conn)? {
        if Path::new(csv_mega_sena_path).exists() {
            println!("Iniciando ingestão do CSV '{}'", csv_mega_sena_path);
            database::csv::ingest_csv_mega_sena_to_sqlite(&mut conn, csv_mega_sena_path)?;
        } else {
            println!(
                "Arquivo CSV '{}' não encontrado — pulando ingestão.",
                csv_mega_sena_path
            );
        }
    }
    return Ok(());
}

pub fn mega_sena_table_has_data(
    conn: &Connection,
) -> Result<bool, rusqlite::Error> {
    let sql = "
        SELECT id
        FROM t_mega_sena
        LIMIT 1;
    ";

    let exists: Option<i32> = conn
        .query_row(sql, [], |row| row.get(0))
        .optional()?;

    if exists.is_some() {
        return Ok(true);
    }


    Ok(false)
}