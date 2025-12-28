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

use database::{csv, migrations, mega_sena_bootstrap};
use engine::mega_sena_service;
use shared::sha3;

use crate::core::mega_sena::MegaSena;
use crate::engine::analyser;

const QTD_TOLERAVEL: u8 = 4;
const PRINT_NAO_JOGAVEL: bool = false;
const QTD_JOGOS_DESEJADOS: u8 = 10;

fn main() -> Result<()> {
    let db_path = "mega_sena.db";
    let csv_mega_sena_path = "mega_sena.csv";

    let mut conn = Connection::open(db_path)?;
    println!("Conectado ao SQLite em {}", db_path);

    if !migrations::check_migration_table_exists(&conn)? {
        println!("Tabela t_migration não existe. Criando...");
        migrations::create_migration_table(&conn)?;
    }

    database::migrations::run_migrations(&conn)?;


    mega_sena_bootstrap::bootstrap_mega_sena_data_from_csv();

    let historico_mega_sela_list = match analyser::listar_historico_mega_sena(&conn) {
        Ok(r) => r,
        Err(_) => panic!("❌ Erro ao carregar histórico da Mega-Sena"),
    };

    let mut jogos_jogaveis_desejados: u8 = QTD_JOGOS_DESEJADOS;
    let mut jogos_gerados: Vec<MegaSena> = Vec::with_capacity(jogos_jogaveis_desejados as usize);

    let mut soma_minima = 346;
    let mut soma_maxima = 0;

    for j in &historico_mega_sela_list {
        if j.generated_by_rust{
            continue;
        }
        let soma = j.bola_1.unwrap_or(0) +
            j.bola_2.unwrap_or(0) +
            j.bola_3.unwrap_or(0) +
            j.bola_4.unwrap_or(0) +
            j.bola_5.unwrap_or(0) +
            j.bola_6.unwrap_or(0);
        if soma < soma_minima {
            soma_minima = soma;
        }

        if soma > soma_maxima {
            soma_maxima = soma;
        }
    }

    println!("\n\n\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Soma minima {}", soma_minima);
    println!("Soma maxima {}", soma_maxima);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");


    while jogos_jogaveis_desejados > 0 {
        let generated_mega_sena: MegaSena = engine::mega_sena_service::generate_mega_sena()?;

        let mut ocorrencias_encontradas = false;


        for h in &historico_mega_sela_list {
            let mut contagem_ocorrencias: u8 = 0;

            for numero in generated_mega_sena.set.clone() {
                if h.set.contains(&numero) {
                    contagem_ocorrencias += 1;
                }
            }

            if PRINT_NAO_JOGAVEL && contagem_ocorrencias >= QTD_TOLERAVEL {
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

            let mut soma_jogo = 0;
            for n in &generated_mega_sena.set {
                soma_jogo+=n;
            }
            if soma_jogo > soma_minima && soma_jogo < soma_maxima {
                jogos_jogaveis_desejados -= 1;
                jogos_gerados.push(generated_mega_sena.clone());
            }
        }
    }

    for jogo in jogos_gerados {
        mega_sena_service::save(&mut conn, jogo.clone())?;
        println!("{}",jogo);
    }

    Ok(())
}
