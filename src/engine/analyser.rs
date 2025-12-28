use std::collections::HashSet;

use rusqlite::{Connection, OptionalExtension, params};

use crate::core::{mega_sena::MegaSena};

/// Verifica se o jogo já existe no histórico
pub fn game_already_exists(
    conn: &Connection,
    megasena: &MegaSena,
) -> Result<bool, rusqlite::Error> {
    let exists: bool = conn.query_row(
        "SELECT EXISTS(
                SELECT 1 FROM t_mega_sena
                WHERE bola_1=?1 AND bola_2=?2 AND bola_3=?3
                  AND bola_4=?4 AND bola_5=?5 AND bola_6=?6
            )",
        params![
            megasena.bola_1,
            megasena.bola_2,
            megasena.bola_3,
            megasena.bola_4,
            megasena.bola_5,
            megasena.bola_6,
        ],
        |row| row.get(0),
    )?;

    Ok(exists)
}

/// Gera todas as combinações de trios (C(6,3)=20) para um jogo de 6 números.
/// O jogo deve ter exatamente 6 números.
pub fn generate_trios(game: &[i64]) -> Vec<[i64; 3]> {
    assert!(game.len() == 6, "O jogo deve conter exatamente 6 números!");

    let mut trios = Vec::new();

    for i in 0..4 {
        for j in (i + 1)..5 {
            for k in (j + 1)..6 {
                trios.push([game[i], game[j], game[k]]);
            }
        }
    }

    trios
}

/// Verifica se algum trio gerado aparece junto em um mesmo concurso do passado.
/// Retorna true se encontrar ao menos 1 ocorrência.
pub fn has_repeated_trio_by_sets(
    conn: &Connection,
    trios: &Vec<[i64; 3]>,
) -> Result<bool, rusqlite::Error> {
    let sql = "
        SELECT concurso
        FROM t_mega_sena
        WHERE bola_1 IN (?1,?2,?3)
           OR bola_2 IN (?1,?2,?3)
           OR bola_3 IN (?1,?2,?3)
           OR bola_4 IN (?1,?2,?3)
           OR bola_5 IN (?1,?2,?3)
           OR bola_6 IN (?1,?2,?3)
        GROUP BY concurso
        HAVING COUNT(*) >= 3
        LIMIT 1;
    ";

    for trio in trios {
        let exists: Option<i32> = conn
            .query_row(sql, params![trio[0], trio[1], trio[2]], |row| row.get(0))
            .optional()?;

        if exists.is_some() {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn listar_historico_mega_sena(
    conn: &Connection,
) -> Result<Vec<MegaSena>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, concurso, data,
                bola_1, bola_2, bola_3,
                bola_4, bola_5, bola_6,
                inserted_at
         FROM t_mega_sena
         ORDER BY concurso ASC",
    )?;

    let rows = stmt.query_map([], |row| {
        // Coleta todas as bolas em um vetor
        let bolas: [Option<i64>; 6] = [
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
        ];

        // Cria e preenche o HashSet
        let mut set = HashSet::new();
        for b in bolas {
            if let Some(v) = b {
                set.insert(v);
            }
        }

        Ok(MegaSena {
            id: row.get(0)?,
            concurso: row.get(1)?,
            data: row.get(2)?,
            bola_1: bolas[0],
            bola_2: bolas[1],
            bola_3: bolas[2],
            bola_4: bolas[3],
            bola_5: bolas[4],
            bola_6: bolas[5],
            inserted_at: row.get(9)?,
            generated_by_rust: false,
            set,
        })
    })?;

    // Coleta tudo para um Vec<HistoricoMegaSena>
    let historico: Vec<MegaSena> = rows
        .filter_map(|r| r.ok()) // descarta linhas com erro
        .collect();

    Ok(historico)
}
