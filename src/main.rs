// src/main.rs
//
// Cargo.toml (adicione estas dependências):
// [dependencies]
// rusqlite = { version = "0.29", features = ["bundled"] }
// sha3 = "0.10"
// rand = "0.8"
// csv = "1.2"
// chrono = { version = "0.4", features = ["alloc", "serde"] }
// anyhow = "1.0"

use anyhow::{Context, Result};
use chrono::Utc;
use rand::seq::IteratorRandom;
use rusqlite::{Connection, OptionalExtension, params};
use std::io::{BufReader, Read};
use std::path::Path;
use std::{fs, num};

pub mod core;
pub mod database;
pub mod engine;
pub mod shared;

use core::mega_sena;
use database::{csv, migrations};
use engine::game_generator;
use shared::sha3;

fn main() -> Result<()> {
    let db_path = "mega_sena.db";
    let csv_path = "mega_sena.csv";

    let mut conn = Connection::open(db_path)?;
    println!("Conectado ao SQLite em {}", db_path);

    if !migrations::check_migration_table_exists(&conn)? {
        println!("Tabela t_migration não existe. Criando...");
        migrations::create_migration_table(&conn)?;
    }

    database::migrations::run_migrations(&conn)?;
    migrations::ensure_historico_table(&conn)?;
    migrations::ensure_generated_table(&conn)?;

    if Path::new(csv_path).exists() {
        println!("Iniciando ingestão do CSV '{}'", csv_path);
        database::csv::ingest_csv_to_sqlite(&mut conn, csv_path)?;
    } else {
        println!(
            "Arquivo CSV '{}' não encontrado — pulando ingestão.",
            csv_path
        );
    }

    for i in 0..100 {
        let generated_mega_sena: mega_sena::MegaSena =
            engine::game_generator::generate_and_store_game(&conn)?;
        engine::game_generator::query_generated_game(&conn, generated_mega_sena.id)?;
        engine::game_generator::query_generated_game_in_history(&conn, &generated_mega_sena)?;
    }
    Ok(())
}
