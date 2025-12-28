

DROP TABLE IF EXISTS t_historico_mega_sena;
DROP TABLE IF EXISTS t_generated_games;
DROP TABLE IF EXISTS t_historico_lotofacil;


CREATE TABLE IF NOT EXISTS t_mega_sena (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            concurso INTEGER NOT NULL,
            data TEXT NOT NULL,
            bola_1 INTEGER, bola_2 INTEGER, bola_3 INTEGER,
            bola_4 INTEGER, bola_5 INTEGER, bola_6 INTEGER,
            inserted_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_h_bola_1 ON t_mega_sena(bola_1);
CREATE INDEX IF NOT EXISTS idx_h_bola_2 ON t_mega_sena(bola_2);
CREATE INDEX IF NOT EXISTS idx_h_bola_3 ON t_mega_sena(bola_3);
CREATE INDEX IF NOT EXISTS idx_h_bola_4 ON t_mega_sena(bola_4);
CREATE INDEX IF NOT EXISTS idx_h_bola_5 ON t_mega_sena(bola_5);
CREATE INDEX IF NOT EXISTS idx_h_bola_6 ON t_mega_sena(bola_6);
