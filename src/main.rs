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

use crate::core::historico_mega_sena::HistoricoMegaSena;
use crate::engine::analyser;

const IS_DADOS_INGERIDOS: bool = true;

fn main() -> Result<()> {
    let db_path = "mega_sena.db";
    let csv_mega_sena_path = "mega_sena.csv";
    let csv_lotofacil_path = "loto_facil.csv";

    let mut conn = Connection::open(db_path)?;
    println!("Conectado ao SQLite em {}", db_path);

    if !migrations::check_migration_table_exists(&conn)? {
        println!("Tabela t_migration não existe. Criando...");
        migrations::create_migration_table(&conn)?;
    }

    database::migrations::run_migrations(&conn)?;

    if !IS_DADOS_INGERIDOS {
        if Path::new(csv_mega_sena_path).exists() {
            println!("Iniciando ingestão do CSV '{}'", csv_mega_sena_path);
            database::csv::ingest_csv_mega_sena_to_sqlite(&mut conn, csv_mega_sena_path)?;
        } else {
            println!(
                "Arquivo CSV '{}' não encontrado — pulando ingestão.",
                csv_mega_sena_path
            );
        }

        if Path::new(csv_lotofacil_path).exists() {
            println!("Iniciando ingestão do CSV '{}'", csv_lotofacil_path);
            database::csv::ingest_csv_lotofacil_to_sqlite(&mut conn, csv_lotofacil_path)?;
        } else {
            println!(
                "Arquivo CSV '{}' não encontrado — pulando ingestão.",
                csv_lotofacil_path
            );
        }
    }

    let historico_mega_sela_list = match analyser::listar_historico_mega_sena(&conn) {
        Ok(r) => r,
        Err(_) => panic!("❌ Erro ao carregar histórico da Mega-Sena"),
    };

    for i in 0..100 {
        let generated_mega_sena = engine::game_generator::generate_mega_sena(&conn)?;

        let mut ocorrencias_encontradas = false;
        const QTD_TOLERAVEL: u8 = 4;

        for h in &historico_mega_sela_list {
            let mut contagem_ocorrencias: u8 = 0;

            for numero in generated_mega_sena.jogo.clone() {
                if h.set.contains(&numero) {
                    contagem_ocorrencias += 1;
                }
            }

            if contagem_ocorrencias >= QTD_TOLERAVEL {
                ocorrencias_encontradas = true;

                println!(
                    "\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
                 🚫 JOGO BLOQUEADO\n\
                 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
                 O jogo {} NÃO deve ser jogado.\n\
                 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
                    generated_mega_sena
                );

                println!("Motivo:");
                println!(
                    "• Pelo menos {QTD_TOLERAVEL} números coincidem com um concurso anterior."
                );
                println!("• Concurso Nº: {}", h.concurso);
                println!("• Bolas do concurso: {}", h);
                println!("• Data: {}", h.data);
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

                break;
            }
        }

        if !ocorrencias_encontradas {
            println!(
                "\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             ✅ JOGO PERMITIDO\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
             O jogo {} pode ser jogado.\n\
             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
                generated_mega_sena
            );
        }
    }

    // if false {
    //     for i in 0..1000000 {
    //         let game_already_existis: bool =
    //             match engine::analyser::game_already_exists(&conn, &generated_mega_sena) {
    //                 Ok(true) => true,
    //                 Ok(false) => false,
    //                 Err(e) => {
    //                     println!("Erro ao verificar: {}", e);
    //                     false
    //                 }
    //             };

    //         let repeated_trio: bool =
    //             match engine::analyser::has_repeated_trio(&conn, &generated_mega_sena) {
    //                 Ok(true) => true,
    //                 Ok(false) => false,
    //                 Err(e) => {
    //                     println!("Erro ao verificar: {}", e);
    //                     false
    //                 }
    //             };

    //         if i % 1000 == 0 {
    //             println!("...");
    //             println!("Iteração [ {} ]", i);
    //             println!("...");
    //         }
    //         if game_already_existis || repeated_trio {
    //             println!("Iteração [ {} ]", i);
    //             println!("---------------------------------------------");
    //             println!("Numeros gerados: {}", generated_mega_sena);

    //             println!("O jogo existe na historia?: {}", game_already_existis);

    //             println!("O jogo possui um trio repetido?: {}", repeated_trio);
    //             println!("---------------------------------------------");
    //         }
    //     }
    // }

    Ok(())
}
