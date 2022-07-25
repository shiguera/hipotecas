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
use umya_spreadsheet::reader::xlsx::XlsxError;
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
    if h.fecha_impago.is_some() {
        h.tabla_amort_impago = h.calcula_tabla_impago();
    }

    print_csv_files(&h);   
    
    // let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
    wait();
        

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
    wait();
    let filename = h.nombre_operacion.clone() + "_euribor";
    let result = h.tabla_amort_con_actualizacion_euribor.print(&filename);
    if result.is_ok() {
        println!("El fichero con la tabla de amortización con actualizaciones del euribor se escribió en {}", h.nombre_operacion.clone()+"_euribor.txt" );
    } else {
        println!("Se produjeron errores al escribir el fichero con la tabla de amortización con actualizaciones del euribor");
        println!("{:?}", result);
    }
    wait();
    if h.fecha_impago.is_some() {
        let filename = h.nombre_operacion.clone() + "_impago";
        let result = h.tabla_amort_impago.print(&filename);
        if result.is_ok() {
            println!("El fichero con la tabla de impagos euribor se escribió en {}", h.nombre_operacion.clone()+"_impago.txt" );
        } else {
            println!("Se produjeron errores al escribir el fichero con la tabla de impagos");
            println!("{:?}", result);
        }
    }
}
fn read_data_from_excel_file(worksheet: &Worksheet) -> Hipoteca {
    let nombre = read_string(worksheet, "C7");
    let fecha = read_fecha(worksheet, "C8");    
    let _meses_primera_cuota = read_i32(worksheet, "C9"); // Pendiente implementación
    let capital = read_f64(worksheet, "C10");
    let tipo = redondea_cinco_decimales(read_f64(worksheet, "C11")/100.0);
    let meses = read_i32(worksheet, "C12");
    let meses_primera_revision = read_i32(worksheet, "C13");
    let intervalo_revisiones = read_i32(worksheet, "C14");
    let incremento_euribor = redondea_cinco_decimales(read_f64(worksheet, "C15")/100.0);
    let i_min = redondea_cinco_decimales(read_f64(worksheet, "C16")/100.0);
    let i_max = redondea_cinco_decimales(read_f64(worksheet, "C17")/100.0);
    let fecha_impago: Option<Date<Utc>> = read_fecha(worksheet, "C18");
    let fecha_resolucion: Option<Date<Utc>> = read_fecha(worksheet, "C19");     
    
    let h = Hipoteca::new(nombre, fecha.unwrap(),
                capital, tipo, meses, 
                meses_primera_revision, intervalo_revisiones,
                incremento_euribor, i_min, i_max, fecha_impago, fecha_resolucion);
    //println!("{} {}", h.fecha_impago.unwrap(), h.fecha_resolucion.unwrap());
    h
}
fn read_string(worksheet: &Worksheet, coordinate: &str) -> String {
    worksheet.get_value(coordinate)
}
fn read_i32(worksheet: &Worksheet, coordinate: &str) -> i32 {
    let cad = worksheet.get_value(coordinate);
    let cap: i32 = cad.parse().unwrap();
    cap    
}
fn read_f64(worksheet: &Worksheet, coordinate: &str) -> f64 {
    let cad = worksheet.get_value(coordinate);
    let cap: f64 = cad.parse().unwrap();
    cap    
}
fn read_fecha(worksheet: &Worksheet, coordinate: &str) -> Option<Date<Utc>> {
    let cell_value = worksheet.get_value(coordinate);
    let fecha_as_f64 = cell_value.parse::<f64>();
    let fecha: Option<Date<Utc>>;
    match fecha_as_f64 {
        Ok(date) => {
            let naive_fecha = 
                excel_to_date_time_object(&date, 
                Some(CALENDAR_WINDOWS_1900.to_owned()));
            //println!("{}", naive_fecha.to_string());
            fecha = Some(Utc.ymd(naive_fecha.year(), naive_fecha.month(), naive_fecha.day()));
        },
        _ => fecha = None
    }
    fecha
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
        assert_eq!(Utc.ymd(2007, 2, 17), h.fecha_escritura);
        assert_eq!(84140.0, h.capital_prestado);
        assert_eq!(0.04, h.tipo_interes_anual);
        assert_eq!(300, h.meses);
        assert_eq!(6, h.meses_hasta_primera_revision);
        assert_eq!(12, h.intervalo_revisiones);
        assert_eq!(0.01, h.incremento_euribor);
        assert_eq!(0.04, h.i_min);
        assert_eq!(0.12, h.i_max);
        assert_eq!(Utc.ymd(2018, 5, 17), h.fecha_impago.unwrap());
        assert_eq!(Utc.ymd(2022, 8, 5), h.fecha_resolucion.unwrap());

        //h.tabla_amort_impago = h.calcula_tabla_impago();
    
    }
    #[test]
    fn test_read_data_from_excel_file_sin_impago() {
        let path = std::path::Path::new("assets\\Libro12.xlsx");
        let book: Spreadsheet = reader::xlsx::read(path).unwrap();
        let worksheet: &Worksheet = book.get_sheet(&0).unwrap();
        
        let h = read_data_from_excel_file(worksheet);
        assert!(h.fecha_impago.is_none());
        assert!(h.fecha_resolucion.is_none());    
    }

    #[test]
    fn test_read_fecha() {
        let path = std::path::Path::new("assets\\libro12.xlsx");
        let book: core::result::Result<Spreadsheet, XlsxError> = reader::xlsx::read(path);
        match book {
            Ok(spreadsheet) => {
                let worksheet: &Worksheet = spreadsheet.get_sheet(&0).unwrap();
                let fecha = read_fecha(worksheet, "C8");
                assert_eq!(Utc.ymd(2007, 02, 17), fecha.unwrap());
                let bad_fecha = read_fecha(worksheet, "D8");
                assert!(bad_fecha.is_none());
            }
            _ => panic!("No se pudo leer la hoja de cálculo")
        }
    }
}
