use crate::mega_sena::MegaSena;
use anyhow::{Context, Result};
use chrono::Utc;
use rand::seq::IteratorRandom;
use rusqlite::{Connection, OptionalExtension, params};
use sha3::{Digest, Sha3_256};
use std::io::{BufReader, Read};
use std::path::Path;
use std::{fs, num};

pub fn generate_and_store_game(conn: &Connection) -> Result<MegaSena> {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i64> = (1..=60)
        .choose_multiple(&mut rng, 6)
        .into_iter()
        .map(|n| n as i64)
        .collect();
    numbers.sort_unstable();

    conn.execute(
        "INSERT INTO t_generated_games (n1, n2, n3, n4, n5, n6, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            numbers[0],
            numbers[1],
            numbers[2],
            numbers[3],
            numbers[4],
            numbers[5],
            Utc::now().naive_utc().to_string()
        ],
    )?;

    let last_id = conn.last_insert_rowid();
    let mut jogo = MegaSena {
        id: last_id,
        jogo: numbers.clone(),
    };
    println!("Jogo gerado e armazenado (id={}): {:?}", last_id, numbers);
    Ok(jogo)
}

/// Consulta um jogo gerado pelo id e imprime.
pub fn query_generated_game(conn: &Connection, id: i64) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, n1, n2, n3, n4, n5, n6, created_at FROM t_generated_games WHERE id = ?1",
    )?;
    let row_opt = stmt
        .query_row(params![id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, String>(7)?,
            ))
        })
        .optional()?;

    if let Some((id, n1, n2, n3, n4, n5, n6, created_at)) = row_opt {
        println!(
            "Jogo id={} gerado em {}: [{}, {}, {}, {}, {}, {}]",
            id, created_at, n1, n2, n3, n4, n5, n6
        );
    } else {
        println!("Nenhum jogo encontrado com id {}", id);
    }

    Ok(())
}

/// Verifica se o jogo gerado já existe no histórico.
pub fn query_generated_game_in_history(conn: &Connection, game: &MegaSena) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT concurso, data, bola_1, bola_2, bola_3, bola_4, bola_5, bola_6
         FROM t_historico_mega_sena
         WHERE bola_1 = ?1 AND bola_2 = ?2 AND bola_3 = ?3
           AND bola_4 = ?4 AND bola_5 = ?5 AND bola_6 = ?6",
    )?;

    let row_opt = stmt
        .query_row(
            params![
                game.jogo[0],
                game.jogo[1],
                game.jogo[2],
                game.jogo[3],
                game.jogo[4],
                game.jogo[5]
            ],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,    // concurso
                    row.get::<_, String>(1)?, // data
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, i64>(7)?,
                ))
            },
        )
        .optional()?;

    if let Some((concurso, data, b1, b2, b3, b4, b5, b6)) = row_opt {
        println!(
            "Jogo já existente no histórico (concurso {} - {}): [{}, {}, {}, {}, {}, {}]",
            concurso, data, b1, b2, b3, b4, b5, b6
        );
    } else {
        println!("Jogo inédito! Nenhum registro encontrado no histórico.");
    }

    Ok(())
}
