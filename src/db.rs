use rusqlite::{Connection, Result};
use chrono::Local;
use crate::models::Movimiento;

pub fn conectar() -> Result<Connection> {
    Connection::open("despeluche.db")
}

pub fn inicializar_db() -> Result<()> {
    let conn = conectar()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS movimientos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            cantidad REAL NOT NULL,
            tipo TEXT NOT NULL CHECK (tipo IN ('ingreso', 'gasto')),
            categoria TEXT NOT NULL,
            asunto TEXT NOT NULL,
            fecha INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn insertar_movimiento(cantidad: f64, tipo: &str, categoria: &str, asunto: &str) -> Result<()> {
    let conn = conectar()?;
    let timestamp = Local::now().timestamp();
    
    conn.execute(
        "INSERT INTO movimientos (cantidad, tipo, categoria, asunto, fecha) VALUES (?1, ?2, ?3, ?4, ?5)",
        (cantidad, tipo, categoria, asunto, timestamp),
    )?;
    Ok(())
}

pub fn obtener_ultimos_movimientos() -> Result<Vec<Movimiento>> {
    let conn = conectar()?;
    let mut stmt = conn.prepare(
        "SELECT id, cantidad, tipo, categoria, asunto, fecha 
         FROM movimientos 
         ORDER BY fecha DESC 
         LIMIT 5"
    )?;
    
    let movimientos_iter = stmt.query_map([], |row| {
        Ok(Movimiento {
            id: Some(row.get(0)?),
            cantidad: row.get(1)?,
            tipo: row.get(2)?,
            categoria: row.get(3)?,
            asunto: row.get(4)?,
            fecha: row.get(5)?,
        })
    })?;

    let movimientos: Result<Vec<_>> = movimientos_iter.collect();
    movimientos
}