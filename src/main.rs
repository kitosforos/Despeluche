use std::io::{self, Write};
use chrono::{Local, TimeZone};
use colored::*;
mod db;
mod models;

const VERSION: &str = "1.0.0";

fn main() {
    if let Err(e) = db::inicializar_db() {
        eprintln!("Error inicializando la base de datos: {}", e);
        return;
    }

    // Título inicial más elegante
    println!("\n{}", "╭────────────────────────╮".bold());
    println!("{}", "│  D E S P E L U C H E   │".bold());
    println!("{}", format!("│        v{}          │", VERSION).bold());
    println!("{}\n", "╰────────────────────────╯".bold());

    loop {
        // Menú principal mejorado
        println!("\n{}", "MENÚ PRINCIPAL".bold());
        println!("{}", "─".repeat(30).dimmed());
        println!("{}. Registrar movimiento", "1".cyan());
        println!("{}. Ver últimos movimientos", "2".cyan());
        println!("{}. Salir\n", "3".cyan());
        
        print!("{} ", "❯".cyan());
        io::stdout().flush().unwrap();

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).expect("Error al leer la opción");

        match opcion.trim() {
            "1" => registrar_movimiento(),
            "2" => mostrar_ultimos_movimientos(),
            "3" => {
                println!("\n{}", "¡Hasta pronto! 👋".bold());
                break;
            }
            _ => println!("{}", "⚠️  Opción no válida".red()),
        }
    }
}

fn registrar_movimiento() {
    println!("\n{}", "REGISTRO DE MOVIMIENTO".bold());
    println!("{}", "─".repeat(30).dimmed());
    println!("{}. Ingreso", "1".cyan());
    println!("{}. Gasto\n", "2".cyan());
    
    print!("{} ", "❯".cyan());
    io::stdout().flush().unwrap();

    let mut tipo = String::new();
    io::stdin().read_line(&mut tipo).expect("Error al leer el tipo");

    let tipo = match tipo.trim() {
        "1" => "ingreso",
        "2" => "gasto",
        _ => {
            println!("{}", "⚠️  Opción no válida".red());
            return;
        }
    };

    // Input mejorado para cantidad
    print!("\n{} ", "Cantidad ❯".cyan());
    io::stdout().flush().unwrap();
    let mut input_cantidad = String::new();
    io::stdin()
        .read_line(&mut input_cantidad)
        .expect("Error al leer la cantidad");
    
    let cantidad: f64 = match input_cantidad.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "⚠️  Por favor, introduce un número válido!".red());
            return;
        }
    };

    // Input mejorado para categoría
    print!("{} ", "Categoría ❯".cyan());
    io::stdout().flush().unwrap();
    let mut categoria = String::new();
    io::stdin()
        .read_line(&mut categoria)
        .expect("Error al leer la categoría");
    
    // Input mejorado para asunto
    print!("{} ", "Asunto ❯".cyan());
    io::stdout().flush().unwrap();
    let mut asunto = String::new();
    io::stdin()
        .read_line(&mut asunto)
        .expect("Error al leer el asunto");

    match db::insertar_movimiento(cantidad, tipo, categoria.trim(), asunto.trim()) {
        Ok(_) => {
            println!("\n{}", "✅ Movimiento registrado correctamente".green().bold());
            println!("{}", "─".repeat(30).dimmed());
        }
        Err(e) => println!("\n{} {}", "❌ Error al registrar el movimiento:".red().bold(), e),
    }
}

fn mostrar_ultimos_movimientos() {
    match db::obtener_ultimos_movimientos() {
        Ok(movimientos) => {
            if movimientos.is_empty() {
                println!("\nNo hay movimientos registrados");
                return;
            }

            println!("\n{}", "Últimos movimientos:".bold());
            for movimiento in movimientos {
                let fecha = Local.timestamp_opt(movimiento.fecha, 0)
                    .single()
                    .unwrap_or_else(|| Local::now())
                    .format("%Y-%m-%d %H:%M:%S");
                
                // Línea separadora
                println!("\n{}", "─".repeat(40).dimmed());
                
                println!("ID: {}", movimiento.id.unwrap());
                
                // Configuramos el color según el tipo
                let (color_str, simbolo) = if movimiento.tipo == "ingreso" {
                    (Color::Green, "↑")
                } else {
                    (Color::Red, "↓")
                };
                
                // Aplicamos el color a la cantidad y al tipo
                let cantidad_str = format!("{:.2}€ {}", movimiento.cantidad, simbolo);
                println!("Cantidad: {}", cantidad_str.color(color_str).bold());
                println!("Tipo: {}", movimiento.tipo.color(color_str).bold());
                
                println!("Categoría: {}", movimiento.categoria);
                println!("Asunto: {}", movimiento.asunto);
                println!("Fecha: {}", fecha.to_string().dimmed());
            }
            
            // Línea separadora final
            println!("{}", "─".repeat(40).dimmed());
        }
        Err(e) => println!("Error al obtener los movimientos: {}", e),
    }
}