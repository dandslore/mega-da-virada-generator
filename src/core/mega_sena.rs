use core::fmt;
use std::collections::HashSet;

use rusqlite::{Result, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MegaSena {
    pub id: i64,
    pub concurso: i64,
    pub data: String,
    pub bola_1: Option<i64>,
    pub bola_2: Option<i64>,
    pub bola_3: Option<i64>,
    pub bola_4: Option<i64>,
    pub bola_5: Option<i64>,
    pub bola_6: Option<i64>,
    pub inserted_at: String,
    pub set: HashSet<i64>,
    pub generated_by_rust: bool,
}

impl<'a> TryFrom<&Row<'a>> for MegaSena {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            concurso: row.get("concurso")?,
            data: row.get("data")?,
            bola_1: row.get("bola_1")?,
            bola_2: row.get("bola_2")?,
            bola_3: row.get("bola_3")?,
            bola_4: row.get("bola_4")?,
            bola_5: row.get("bola_5")?,
            bola_6: row.get("bola_6")?,
            inserted_at: row.get("inserted_at")?,
            generated_by_rust: row.get("generated_by_rust")?,
            set: HashSet::new(),
        })
    }
}

impl fmt::Display for MegaSena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",
            self.bola_1.unwrap_or(99),
            self.bola_2.unwrap_or(99),
            self.bola_3.unwrap_or(99),
            self.bola_4.unwrap_or(99),
            self.bola_5.unwrap_or(99),
            self.bola_6.unwrap_or(99)
        )
    }
}
