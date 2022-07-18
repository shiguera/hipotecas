mod libs;

use chrono::prelude::*;
use chrono::Utc;
use libs::hipoteca::*;
use libs::lib::redondea_cinco_decimales;
use libs::lib::redondea_dos_decimales;
use std::io;
use std::io::*;
use std::io::Result;
use colored::*;


fn main() -> Result<()>{
    control::set_virtual_terminal(true);
    disp_caratula();
    let nombre = read_nombre();
    //println!("{}", nombre);
    let fecha: Date<Utc> = read_fecha();
    //println!("{}", fecha);
    let capital = read_capital();
    //println!("{}", capital);
    let tipo: f64 = read_tipo_interes();
    //println!("{}", tipo);
    let meses: i32 = read_meses();
    //println!("{}", meses);
    let meses_primera_revision = read_meses_primera_revision();
    //println!("{}", meses_primera_revision);
    let intervalo_revisiones = read_intervalo_revisiones(); 
    //println!("{}", intervalo_revisiones);
    let incremento_euribor = read_incremento_euribor();
    //println!("{}", incremento_euribor);
    let i_min = read_i_min();
    //println!("{}", i_min);
    let i_max = read_i_max();
    //println!("{}", i_max);

    let mut h = Hipoteca::new(nombre, fecha,
         capital, tipo, meses, 
         meses_primera_revision, intervalo_revisiones,
          incremento_euribor, i_min, i_max);
    
    h.tabla_amort_sin_actualizacion = h.calcula_tabla_amort_sin_actualizacion();
    //println!("tabla 1");
    h.tabla_amort_con_actualizacion_euribor = h.calcula_tabla_amort_con_actualizacion_euribor();
    //println!("tabla 2");
    let filename = h.nombre_operacion.clone();
    let result = h.tabla_amort_sin_actualizacion.print(&filename);
    if result.is_ok() {
        println!("El fichero con la tabla de amortización inicial se escribió en {}", h.nombre_operacion.clone()+".txt" );
    } else {
        println!("Se produjeron errores al escribir el fichero con la tabla de amortización inicial");
        println!("{:?}", result);
    }
    let filename = h.nombre_operacion.clone() + "_euribor";
    let result = h.tabla_amort_con_actualizacion_euribor.print(&filename);
    if result.is_ok() {
        println!("El fichero con la tabla de amortización con actualizaciones del euribor se escribió en {}", h.nombre_operacion.clone()+"_euribor.txt" );
    } else {
        println!("Se produjeron errores al escribir el fichero con la tabla de amortización con actualizaciones del euribor");
        println!("{:?}", result);
    }
    Ok(())
}
fn read_i_max() -> f64 {
    let mut cad = String::new();
    while cad.len() == 0 {
        println!("{}", "Tipo máximo, en tanto por ciento, establecido en las cláusulas de la hipoteca");
        print!("{}", "Tipo máximo: ".green());
        io::stdout().flush() ;    
        let result = io::stdin().read_line(&mut cad);
    }
    let i_max = cad.trim().replace(",", ".").parse::<f64>().unwrap();
    redondea_cinco_decimales(i_max/100.0)
}
fn read_i_min() -> f64 {
    let mut cad = String::new();
    while cad.len() == 0 {
        println!("{}", "Tipo mínimo en tanto por ciento establecido en las cláusulas de la hipoteca");
        print!("{}", "Tipo mínimo: ".green());
        io::stdout().flush() ;    
        let result = io::stdin().read_line(&mut cad);
    }
    let i_min = cad.trim().replace(",", ".").parse::<f64>().unwrap();
    redondea_cinco_decimales(i_min/100.0)
}
fn read_incremento_euribor() -> f64 {
    let mut cad = String::new();
    while cad.len() == 0 {
        println!("{}", "Tanto por ciento de incremento sobre el euribor en las revisiones.");
        print!("{}", "Incremento Euribor: ".green());
        io::stdout().flush() ;    
        let result = io::stdin().read_line(&mut cad);
    }
    let inc = cad.trim().replace(",", ".").parse::<f64>().unwrap();
    redondea_cinco_decimales(inc/100.0)
}
fn read_intervalo_revisiones() -> i32 {
    let mut cad = String::new();
    while cad.len() == 0 {
        println!("{}", "Meses entre revisiones de tipos");
        print!("{}", "Intervalo revisiones: ".green());
        io::stdout().flush() ;    
        let result = io::stdin().read_line(&mut cad);
    }
    let meses = cad.trim().parse::<i32>().unwrap();
    meses
}
fn read_meses_primera_revision() -> i32 {
    let mut cad = String::new();
    while cad.len() == 0 {
        println!("{}", "Meses hasta la primera revisión de tipos");
        print!("{}", "Meses hasta primera revisión: ".green());
        io::stdout().flush() ;    
        let result = io::stdin().read_line(&mut cad);
    }
    let meses = cad.trim().parse::<i32>().unwrap();
    meses
}
fn read_meses() -> i32 {
    let mut cad = String::new();
    while cad.len() == 0 {
        println!("{}", "Meses de duaración del préstamo. Por ejemplo, 20 años son 240 meses");
        print!("{}", "Meses: ".green());
        io::stdout().flush() ;    
        let result = io::stdin().read_line(&mut cad);
    }
    let meses = cad.trim().parse::<i32>().unwrap();
    meses
}
fn read_tipo_interes() -> f64 {
    let mut cad = String::new();
    while cad.len() == 0 {
        println!("{}", "Tipo de interés en tanto por ciento. Por ejemplo, si el tipo es del 4,5%, teclee 4,5");
        print!("{}", "Tipo de interes nominal anual: ".green());
        io::stdout().flush() ;    
        let result = io::stdin().read_line(&mut cad);
    }
    let tipo = cad.trim().replace(",", ".").parse::<f64>().unwrap();
    redondea_cinco_decimales(tipo/100.0)
}
fn read_capital() -> f64 {
    let mut cad = String::new();
    while cad.len() == 0 {
        print!("{}", "Capital del préstamo: ".green());
        io::stdout().flush() ;    
        let result = io::stdin().read_line(&mut cad);
    }
    let capital = cad.trim().replace(",", ".").parse::<f64>().unwrap();
    redondea_dos_decimales(capital)
}
fn read_fecha() -> Date<Utc> {
    println!("La fecha corresponde a la fecha de la hipoteca.");
    println!("Se leerán de manera separada los números del año, mes y día");

    let mut cad = String::new();
    while cad.len() == 0 {
        print!("{}", "Año: ".green());
        io::stdout().flush() ;    
        let result = io::stdin().read_line(&mut cad);
    }
    let year = cad.trim().parse::<i32>().unwrap();
    
    let mut cad = String::new();
    while cad.len() == 0 {
        print!("{}", "Mes (1 a 12): ".green());
        io::stdout().flush() ;
        let result = io::stdin().read_line(&mut cad);
    }
    let month = cad.trim().parse::<u32>().unwrap();

    let mut cad = String::new();
    while cad.len() == 0 {
        print!("{}", "Día (1 a 31): ".green());
        io::stdout().flush() ;
        let result = io::stdin().read_line(&mut cad);
    }
    let day = cad.trim().parse::<u32>().unwrap();
    
    Utc.ymd(year, month, day)
}
fn read_nombre() -> String {
    println!("{}", "El nombre de la operación se utilizará para dar nombre a los ficheros con las tablas de amortización.");
    println!("Utilice nombres diferentes en cada operación, o se sobrescribirán los ficheros.");
    println!("No utilice espacios ni caracteres especiales en los nombres, solo letras del alfabeto inglés, ");
    println!("números y guión bajo _. Un ejemplo podría ser h_1.");
    print!("{}", "Nombre de la operación: ".green());
    io::stdout().flush() ;
    let mut nombre = String::new();
    let result = io::stdin().read_line(&mut nombre);
    nombre.trim().to_string()
}
fn disp_caratula() {
    //print!("\x1B[2J\x1B[1;1H");
    println!("{}", "---------------------------------------------------------------".on_blue().bright_white().bold());
    println!("{}", "   CÁLCULO DE HIPOTECAS                                        ".on_blue().white().bold());
    println!("{}", "   (c) Gestoría Montalvo                                       ".on_blue().white().bold());
    println!("{}", "---------------------------------------------------------------".on_blue().white().bold());    
    println!();
}