mod libs;


use chrono::prelude::*;
use chrono::Utc;
use libs::hipoteca::*;
use libs::lib::*;
use std::io;
use std::io::*;
use std::io::Result;
use colored::*;
use std::env::args;


use umya_spreadsheet::*;
use umya_spreadsheet::helper::date::{excel_to_date_time_object, CALENDAR_WINDOWS_1900};
use std::ops::Deref;

fn main() -> Result<()>{
    if args().len() != 2 {
        println!("ERROR DE EJECUCIÓN");
    }
    let worksheet_file_name: String = args().last().unwrap();
    let working_directory: String = String::from("C:\\Users\\profesor\\rustprj\\hipotecas\\target\\debug\\"); 

    let filepath_cad: String =  working_directory + &worksheet_file_name;
    let path = std::path::Path::new(&filepath_cad);
    let book: Spreadsheet = reader::xlsx::read(path).unwrap();
    let worksheet: &Worksheet = book.get_sheet(&0).unwrap();
    
    let mut h = read_data_from_excel_file(worksheet);
    
    h.tabla_amort_sin_actualizacion = h.calcula_tabla_amort_sin_actualizacion();
    h.tabla_amort_con_actualizacion_euribor = h.calcula_tabla_amort_con_actualizacion_euribor();

    print_csv_files(&h);    


    // let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
    // wait();
        

    Ok(())
}

fn date_to_string(date: Date<Utc>) -> String {
    let fecha = date.day().to_string() + "/" + &date.month().to_string() + &"/" +
        &date.year().to_string();
    fecha
}

fn print_csv_files(h: &Hipoteca) {
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
}
fn read_data_from_excel_file(worksheet: &Worksheet) -> Hipoteca {
    let nombre = read_string(worksheet, "C5");
    let fecha = read_fecha(worksheet, "C6");    
    let meses_primera_cuota = read_i32(worksheet, "C7");
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
    let cell = worksheet.get_cell("C6").unwrap();
    let fecha_excel = cell.get_value().deref().to_owned();
    let fecha_f64 = fecha_excel.parse().unwrap();
    let fecha = excel_to_date_time_object(&fecha_f64,
         Some(CALENDAR_WINDOWS_1900.to_owned()));
    let fecha_2 = Utc.ymd(fecha.year(), fecha.month(), fecha.day());
    fecha_2
}
fn wait() {
    let mut nombre = String::new();
    println!("Nombre: ");
    let result = io::stdin().read_line(&mut nombre);
}

