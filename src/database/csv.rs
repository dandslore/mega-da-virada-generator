use anyhow::{Context, Result};
use rusqlite::{Connection, params};

pub fn ingest_csv_mega_sena_to_sqlite(conn: &mut Connection, csv_path: &str) -> Result<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path(csv_path)
        .with_context(|| format!("Falha ao abrir CSV {}", csv_path))?;

    let tx = conn.transaction()?;
    let mut inserted = 0usize;

    for result in rdr.records() {
        let record = result?;
        if record.len() < 8 {
            eprintln!("Linha ignorada (colunas insuficientes): {:?}", record);
            continue;
        }

        let strip_quotes = |s: &str| -> String {
            s.trim()
                .trim_matches('\'')
                .trim_matches('"')
                .trim()
                .to_string()
        };

        let concurso: i64 = strip_quotes(&record[0]).parse()?;
        let data = strip_quotes(&record[1]);
        let bola_1: i64 = strip_quotes(&record[2]).parse()?;
        let bola_2: i64 = strip_quotes(&record[3]).parse()?;
        let bola_3: i64 = strip_quotes(&record[4]).parse()?;
        let bola_4: i64 = strip_quotes(&record[5]).parse()?;
        let bola_5: i64 = strip_quotes(&record[6]).parse()?;
        let bola_6: i64 = strip_quotes(&record[7]).parse()?;

        tx.execute(
            "INSERT INTO t_mega_sena
                (concurso, data, bola_1, bola_2, bola_3, bola_4, bola_5, bola_6)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                concurso, data, bola_1, bola_2, bola_3, bola_4, bola_5, bola_6
            ],
        )?;
        inserted += 1;
    }

    tx.commit()?;
    println!("Ingestão concluída. {} linhas inseridas.", inserted);
    Ok(())
}