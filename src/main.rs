use std::io::{self, Write};

struct Gasto {
    cantidad: f64,
    categoria: String,
}

fn main() {
    let mut gasto: Option<Gasto> = None;

    println!("Bienvenido a Despeluche!");

    loop {
        println!("\n1. Registrar gasto");
        println!("2. Ver último gasto");
        println!("3. Salir\n");
        
        print!("> ");
        io::stdout().flush().unwrap();

        let mut opcion =  String::new();
        io::stdin()
            .read_line(&mut opcion)
            .expect("Error");

        match opcion.trim() {
            "1" => {
                print!("\nIntroduce cantidad > ");
                io::stdout().flush().unwrap();
                let mut input_cantidad = String::new();
                io::stdin()
                    .read_line(&mut input_cantidad)
                    .expect("Gasto mal introducido");
                
                let input_cantidad: f64 = match input_cantidad.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Por favor, introduce un número válido!");
                        continue;
                    }
                };

                print!("Introduce categoría > ");
                io::stdout().flush().unwrap();
                let mut input_categoria = String::new();
                io::stdin()
                    .read_line(&mut input_categoria)
                    .expect("Gasto mal introducido");
                
                let input_categoria = input_categoria.trim().to_string();

                gasto = Some(Gasto {
                    cantidad: input_cantidad,
                    categoria: input_categoria,
                });
            
            }
            "2" => {
                match gasto {
                    Some(ref g) => {
                        println!("\nÚltimo gasto:\n");
                        println!("\tCategoría: {}", g.categoria);
                        println!("\tCantidad: {}", g.cantidad);
                    }
                    None => {
                        println!("\nNo hay gasto registrado");
                    }
                }
            }
            "3" => {
                println!("\nHasta pronto!");
                break;
            }
            _ => println!("Opción no válida"),
        }
    }
}