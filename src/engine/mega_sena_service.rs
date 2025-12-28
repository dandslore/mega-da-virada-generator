use crate::core::mega_sena::MegaSena;
use anyhow::{Context, Result};
use chrono::Utc;
use rand::seq::IteratorRandom;
use rusqlite::{Connection, OptionalExtension, params};
use sha3::{Digest, Sha3_256};
use std::collections::HashSet;
use std::io::{BufReader, Read};
use std::path::Path;
use std::{fs, num};
use uuid::{uuid, Uuid};

pub fn generate_mega_sena() -> Result<MegaSena> {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i64> = (1..=60)
        .choose_multiple(&mut rng, 6)
        .into_iter()
        .map(|n| n as i64)
        .collect();
    numbers.sort_unstable();

    let mut set = HashSet::new();
    for n in &numbers {
        set.insert(n.clone());
    }

    Ok(MegaSena {
        id: 0,
        concurso: 999999,
        data: String::from("31/12/2025"),
        bola_1: Option::from(numbers[0]),
        bola_2: Option::from(numbers[1]),
        bola_3: Option::from(numbers[2]),
        bola_4: Option::from(numbers[3]),
        bola_5: Option::from(numbers[4]),
        bola_6: Option::from(numbers[5]),
        inserted_at: String::from("Algum momento"),
        generated_by_rust: true,
        set: set.clone()
    }
    )

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
         FROM t_mega_sena
         WHERE bola_1 = ?1 AND bola_2 = ?2 AND bola_3 = ?3
           AND bola_4 = ?4 AND bola_5 = ?5 AND bola_6 = ?6",
    )?;

    let row_opt = stmt
        .query_row(
            params![
                game.bola_1,
                game.bola_2,
                game.bola_3,
                game.bola_4,
                game.bola_5,
                game.bola_6
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

pub fn save(conn: &mut Connection, mega_sena: MegaSena) -> Result<()> {

    let tx = conn.transaction()?;

    let concurso = Uuid::new_v4().to_string();
    let data = chrono::Local::now().format("%d/%m/%Y").to_string();
    let data_typesafe = chrono::Local::now().format("%Y-%m-%d").to_string();
    let bola_1 = mega_sena.bola_1;
    let bola_2 = mega_sena.bola_2;
    let bola_3 = mega_sena.bola_3;
    let bola_4 = mega_sena.bola_4;
    let bola_5 = mega_sena.bola_5;
    let bola_6 = mega_sena.bola_6;
    let generated_by_rust = mega_sena.generated_by_rust;

    tx.execute(
        "INSERT INTO t_mega_sena
            (concurso, data, data_typesafe, bola_1, bola_2, bola_3, bola_4, bola_5, bola_6, generated_by_rust)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            concurso, data, data_typesafe, bola_1, bola_2, bola_3, bola_4, bola_5, bola_6, generated_by_rust
        ],
    )?;

    tx.commit()?;
    Ok(())
}
