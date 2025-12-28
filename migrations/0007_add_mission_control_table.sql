CREATE TABLE t_mission_control
(
    id                INTEGER
        PRIMARY KEY AUTOINCREMENT,
    key          TEXT NOT NULL,
    inserted_at       TEXT    DEFAULT (DATETIME('now')),
    enable,
    generated_by_rust BOOLEAN DEFAULT FALSE
);