mod libs;

use std::io;
use std::io::{prelude::*, Result};

use chrono::prelude::*;
use chrono::Utc;
use libs::hipoteca::*;
use libs::lib::*;
// use colored::*;

use std::env::args;


use umya_spreadsheet::*;
use umya_spreadsheet::helper::date::{excel_to_date_time_object, CALENDAR_WINDOWS_1900};
use std::ops::Deref;

fn main() -> Result<()>{
    if args().len() != 2 {
        println!("ERROR DE EJECUCIÓN");
        wait();
    }
    let worksheet_file_name: String = args().last().unwrap();
    // println!("{}", worksheet_file_name);
    
    //let working_directory: String = String::from("C:\\ProgramaHipotecas\\");
    let working_directory: String = String::from("C:\\Users\\profesor\\rustprj\\hipotecas\\target\\debug\\");
     
    //println!("{}", working_directory);
    let filepath_cad: String =  working_directory + &worksheet_file_name;
    //println!("{}", filepath_cad);
    let path = std::path::Path::new(&filepath_cad);
    let book: Spreadsheet = reader::xlsx::read(path).unwrap();
    let worksheet: &Worksheet = book.get_sheet(&0).unwrap();
    
    let mut h = read_data_from_excel_file(worksheet);
    
    //println!("Leídos datos");
    h.tabla_amort_impago = h.calcula_tabla_impago();

    print_csv_files(&h);   
    
    // let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
    //wait();
        

    Ok(())
}


fn print_csv_files(h: &Hipoteca) {
    let filename = h.nombre_operacion.clone();
    println!("{}", filename);
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
    let filename = h.nombre_operacion.clone() + "_impago";
    let result = h.tabla_amort_impago.print(&filename);
    if result.is_ok() {
        println!("El fichero con la tabla de impagos euribor se escribió en {}", h.nombre_operacion.clone()+"_impago.txt" );
    } else {
        println!("Se produjeron errores al escribir el fichero con la tabla de impagos");
        println!("{:?}", result);
    }
}
fn read_data_from_excel_file(worksheet: &Worksheet) -> Hipoteca {
    let nombre = read_string(worksheet, "C5");
    let fecha = read_fecha(worksheet, "C6");    
    let _meses_primera_cuota = read_i32(worksheet, "C7"); // Pendiente implementación
    let capital = read_f64(worksheet, "C8");
    let tipo = redondea_cinco_decimales(read_f64(worksheet, "C9")/100.0);
    let meses = read_i32(worksheet, "C10");
    let meses_primera_revision = read_i32(worksheet, "C11");
    let intervalo_revisiones = read_i32(worksheet, "C12");
    let incremento_euribor = redondea_cinco_decimales(read_f64(worksheet, "C13")/100.0);
    let i_min = redondea_cinco_decimales(read_f64(worksheet, "C14")/100.0);
    let i_max = redondea_cinco_decimales(read_f64(worksheet, "C15")/100.0);
    let fecha_impago: Date<Utc> = read_fecha(worksheet, "C16");
    let fecha_resolucion: Date<Utc> = read_fecha(worksheet, "C17");     
    
    let h = Hipoteca::new(nombre, fecha,
                capital, tipo, meses, 
                meses_primera_revision, intervalo_revisiones,
                incremento_euribor, i_min, i_max, fecha_impago, fecha_resolucion);
    println!("{} {}", h.fecha_impago, h.fecha_resolucion);
    h
}
fn read_string(worksheet: &Worksheet, coordinate: &str) -> String {
    worksheet.get_cell(coordinate).unwrap().get_value().deref().to_owned()
}
fn read_i32(worksheet: &Worksheet, coordinate: &str) -> i32 {
    let cad = worksheet.get_cell(coordinate).unwrap().get_value().deref().to_owned();
    let cap: i32 = cad.parse().unwrap();
    cap    
}
fn read_f64(worksheet: &Worksheet, coordinate: &str) -> f64 {
    let cad = worksheet.get_cell(coordinate).unwrap().get_value().deref().to_owned();
    let cap: f64 = cad.parse().unwrap();
    cap    
}
fn read_fecha(worksheet: &Worksheet, coordinate: &str) -> Date<Utc> {
    let cell = worksheet.get_cell(coordinate).unwrap();
    let fecha_excel = cell.get_value().deref().to_owned();
    let fecha_f64 = fecha_excel.parse().unwrap();
    let fecha = excel_to_date_time_object(&fecha_f64,
         Some(CALENDAR_WINDOWS_1900.to_owned()));
    let fecha_2 = Utc.ymd(fecha.year(), fecha.month(), fecha.day());
    fecha_2
}

/// Se utiliza para parar el programa en la terminal antes de volver a excel 
#[allow(dead_code)]
fn wait() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_data_from_excel_file() {
        let path = std::path::Path::new("assets\\Libro11.xlsx");
        let book: Spreadsheet = reader::xlsx::read(path).unwrap();
        let worksheet: &Worksheet = book.get_sheet(&0).unwrap();
        
        let h = read_data_from_excel_file(worksheet);
        println!("Leídos datos");
        assert_eq!("Libro11", h.nombre_operacion);
        assert_eq!(Utc.ymd(2004, 2, 17), h.fecha_escritura);
        assert_eq!(84140.0, h.capital_prestado);
        assert_eq!(0.04, h.tipo_interes_anual);
        assert_eq!(300, h.meses);
        assert_eq!(6, h.meses_hasta_primera_revision);
        assert_eq!(12, h.intervalo_revisiones);
        assert_eq!(0.01, h.incremento_euribor);
        assert_eq!(0.04, h.i_min);
        assert_eq!(0.12, h.i_max);
        assert_eq!(Utc.ymd(2018, 5, 17), h.fecha_impago);
        assert_eq!(Utc.ymd(2022, 8, 5), h.fecha_resolucion);

        //h.tabla_amort_impago = h.calcula_tabla_impago();
    
    }
    #[test]
    fn test_read_data_from_excel_file_sin_impago() {
        let path = std::path::Path::new("assets\\Libro12.xlsx");
        let book: Spreadsheet = reader::xlsx::read(path).unwrap();
        let worksheet: &Worksheet = book.get_sheet(&0).unwrap();
        
        let h = read_data_from_excel_file(worksheet);
        println!("Leídos datos");
        assert_eq!("Libro11", h.nombre_operacion);
        assert_eq!(Utc.ymd(2004, 2, 17), h.fecha_escritura);
        assert_eq!(84140.0, h.capital_prestado);
        assert_eq!(0.04, h.tipo_interes_anual);
        assert_eq!(300, h.meses);
        assert_eq!(6, h.meses_hasta_primera_revision);
        assert_eq!(12, h.intervalo_revisiones);
        assert_eq!(0.01, h.incremento_euribor);
        assert_eq!(0.04, h.i_min);
        assert_eq!(0.12, h.i_max);
        println!("Fecha impago:{}", h.fecha_impago);
        //assert_eq!(Utc.ymd(2018, 5, 17), h.fecha_impago);
        //assert_eq!(Utc.ymd(2022, 8, 5), h.fecha_resolucion);

        //h.tabla_amort_impago = h.calcula_tabla_impago();
    
    }
}
