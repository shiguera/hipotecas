mod libs;

use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

//use std::error::Error;
use std::io;
use std::io::{prelude::*, Result};
use chrono::prelude::*;
use chrono::Utc;
use libs::novacion::Novacion;
use libs::{hipoteca::*, novacion::*};
use libs::lib::*;
// use colored::*;
use std::env::args;
use umya_spreadsheet::*;
use umya_spreadsheet::reader::xlsx::XlsxError;
use umya_spreadsheet::helper::date::{excel_to_date_time_object, CALENDAR_WINDOWS_1900};
//use std::ops::Deref;

/// Número máximo de novaciones que se leerán de la hoja de cálculo
const MAX_NOVACIONES: i32 = 5;

/// El directorio de trabajo del programa. Es donde esté alojada la hoja de cálculo con los datos
/// de la hipoteca a procesar
//const WORKING_DIRECTORY: String = String::from("C:\\ProgramaHipotecas\\");
const WORKING_DIRECTORY: &str = "C:\\Users\\profesor\\rustprj\\hipotecas\\target\\debug\\";

fn main() -> Result<()>{
    if args().len() != 2 {
        println!("ERROR DE EJECUCIÓN: El programa no ha recibido el número correcto de parámetros");
        wait();
        return Ok(())
    }

    configure_logger();
    debug!("WORKING_DIRECTORY: {}", WORKING_DIRECTORY);

    let worksheet_file_name: String = args().last().unwrap();
    debug!("Hoja de calculo: {}", worksheet_file_name);
    
    let filepath_cad: String =  String::from(WORKING_DIRECTORY) + &worksheet_file_name;
    let book: Spreadsheet = open_workbook(filepath_cad);

    let worksheet: &Worksheet = book.get_sheet(&0).unwrap();
    
    // Leer los datos básicos de la hipoteca desde la hoja de cálculo
    let mut h = read_data_from_excel_file(worksheet);
    debug!("Leidos datos de la hipoteca");

    // Hacer el cáclculo de las novaciones
    h.calcula_novaciones();
    debug!("Calculadas novaciones");
    
    // Hacer el cáclculo del impago
    if h.fecha_impago.is_some() {
        debug!("Calculando tabla de impago");
        h.tabla_amort_impago = h.calcula_tabla_impago();
        debug!("Calculada tabla impago");
    }

    // Imprimir los resultados en ficheros csv
    print_csv_files(&h);   
    
    // let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
    wait();
        

    Ok(())
}

fn configure_logger() -> std::result::Result<(), SetLoggerError> {
    let level = log::LevelFilter::Info;
    let file_path = "hipotecas.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M)}-{f}-{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?;

    debug!("------------------");
    debug!("Logger initialized");
    debug!("------------------");

    Ok(())
}
fn open_workbook(filepath_cad: String) -> Spreadsheet {
    debug!("fn open_workbook()");
    let path = std::path::Path::new(&filepath_cad);
    let book: Spreadsheet;
    let book_option = reader::xlsx::read(path);
    match book_option {
        Ok(_) => {
            book = book_option.unwrap();
            debug!("Leida hoja de calculo: {:?}", filepath_cad);
            book
        },
        _ => {
            error!("No se pudo leer la hoja de calculo {}", filepath_cad);
            println!("Error al leer hoja de cálculo"); 
            wait(); 
            std::process::exit(0);
        }
    }
}
fn print_csv_files(h: &Hipoteca) {
    debug!("fn print_csv_files()");
    let filename = String::from(WORKING_DIRECTORY) + &h.nombre_operacion.clone() + ".txt";
    let result = h.tabla_amort_sin_actualizacion.print(&filename);
    if result.is_ok() {
        println!("El fichero con la tabla de amortización inicial se escribió en {}", filename.clone());
        debug!("El fichero con la tabla de amortizacion inicial se escribio en {}", filename);
    } else {
        println!("Se produjeron errores al escribir el fichero con la tabla de amortización inicial");
        println!("{:?}", result);
        error!("Se produjeron errores al escribir el fichero con la tabla de amortizacion inicial: {:?}", result);
    }
    wait();
    let filename = String::from(WORKING_DIRECTORY) + &h.nombre_operacion.clone() + "_euribor.txt";
    let result = h.tabla_amort_con_actualizacion_euribor.print(&filename);
    if result.is_ok() {
        println!("El fichero con la tabla de amortización con actualizaciones del euribor se escribió en {}", filename.clone() );
        debug!("El fichero con la tabla de amortizacion con actualizaciones del euribor se escribio en {}", filename);
    } else {
        println!("Se produjeron errores al escribir el fichero con la tabla de amortización con actualizaciones del euribor");
        println!("{:?}", result);
        error!("Se produjeron errores al escribir el fichero con la tabla de amortizacion con actualizaciones del euribor: {:?}", result);
    }
    wait();
    if h.fecha_impago.is_some() {
        let filename = String::from(WORKING_DIRECTORY) + &h.nombre_operacion.clone() + "_impago.txt";
        let result = h.tabla_amort_impago.print(&filename);
        if result.is_ok() {
            println!("El fichero con la tabla de impagos se escribió en {}", filename.clone());
            debug!("El fichero con la tabla de impagos se escribio en {}", filename);
        } else {
            println!("Se produjeron errores al escribir el fichero con la tabla de impagos");
            println!("{:?}", result);
            error!("Se produjeron errores al escribir el fichero con la tabla de impagos{:?}", result);
        }
    }
}
fn read_data_from_excel_file(worksheet: &Worksheet) -> Hipoteca {
    debug!("fn read_data_from_excel_files()");
    let nombre = read_string(worksheet, "C7");
    let fecha = read_fecha(worksheet, "C8");    
    let fecha_primera_cuota = read_fecha(worksheet, "C9"); 
    let capital = redondea_dos_decimales(read_f64(worksheet, "C10"));
    let tipo = redondea_cinco_decimales(read_f64(worksheet, "C11")/100.0);
    let meses = read_i32(worksheet, "C12");
    let meses_primera_revision = read_i32(worksheet, "C13");
    let intervalo_revisiones = read_i32(worksheet, "C14");
    let incremento_euribor = redondea_cinco_decimales(read_f64(worksheet, "C15")/100.0);
    let i_min = redondea_cinco_decimales(read_f64(worksheet, "C16")/100.0);
    let i_max = redondea_cinco_decimales(read_f64(worksheet, "C17")/100.0);
    let fecha_impago: Option<Date<Utc>> = read_fecha(worksheet, "C18");
    let fecha_resolucion: Option<Date<Utc>> = read_fecha(worksheet, "C19");     
    
    let mut h = Hipoteca::new(nombre, fecha.unwrap(),
        fecha_primera_cuota.unwrap(), capital, tipo, meses, 
        meses_primera_revision, intervalo_revisiones,
        incremento_euribor, i_min, i_max, fecha_impago, fecha_resolucion);
    h.novaciones = read_novaciones(worksheet);
    h
}

fn read_novaciones(worksheet: &Worksheet) -> Vec<Novacion> {
    debug!("fn read_novaciones()");
    let mut novaciones = Vec::<Novacion>::new();
    let row: u32 = 9; // Primera fila de datos de las novaciones en la hoja de cálculo
    let mut col: u32 = 6; // Primera columna de datos de las novaciones en la hoja de cálculo
    for _i in 0..MAX_NOVACIONES {
        let fecha: Option<Date<Utc>> = read_fecha_by_column_and_row(worksheet, &col, &row);
        if fecha.is_some() {
            let incremento_capital = read_f64_by_column_and_row(worksheet, &col, &(row+1));
            let novacion: Novacion = Novacion::new(fecha.unwrap(), incremento_capital);
            novaciones.push(novacion);
        }
        col += 1;
    }
    novaciones
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
    let cad: String = worksheet.get_value(coordinate);
    let cap = cad.parse();
    match cap {
        Ok(number) => number,
        _ => 0.0,
    }
}
fn read_f64_by_column_and_row(worksheet: &Worksheet, col: &u32, row: &u32) -> f64 {
    let cad: String = worksheet.get_value_by_column_and_row(col, row);
    let cap = cad.parse();
    match cap {
        Ok(number) => number,
        _ => 0.0,
    }
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
fn read_fecha_by_column_and_row(worksheet: &Worksheet, col: &u32, row: &u32) -> Option<Date<Utc>> {
    let cell_value = worksheet.get_value_by_column_and_row(col, row);
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
    fn test_read_novaciones() {
        let path = std::path::Path::new("assets\\Libro12.xlsx");
        let book: Spreadsheet = reader::xlsx::read(path).unwrap();
        let worksheet: &Worksheet = book.get_sheet(&0).unwrap(); 
        let h = read_data_from_excel_file(worksheet);
        
        assert_eq!(2, h.novaciones.len());
        assert_eq!(Utc.ymd(2011, 11, 5), h.novaciones[0].fecha_novacion);
        assert_eq!(20000.0, h.novaciones[0].incremento_capital);
        assert_eq!(Utc.ymd(2021, 12, 16), h.novaciones[1].fecha_novacion);
        assert_eq!(32500.0, h.novaciones[1].incremento_capital);
    }
    #[test]
    fn test_read_data_from_excel_file() {
        let path = std::path::Path::new("assets\\Libro11.xlsx");
        let book: Spreadsheet = reader::xlsx::read(path).unwrap();
        let worksheet: &Worksheet = book.get_sheet(&0).unwrap();
        
        let h = read_data_from_excel_file(worksheet);
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
                assert_eq!(Utc.ymd(2007, 3, 17), fecha.unwrap());
                let bad_fecha = read_fecha(worksheet, "D8");
                assert!(bad_fecha.is_none());
            }
            _ => panic!("No se pudo leer la hoja de cálculo")
        }
    }
    #[test]
    fn test_read_fecha_by_column_and_row() {
        let path = std::path::Path::new("assets\\libro12.xlsx");
        let book: core::result::Result<Spreadsheet, XlsxError> = reader::xlsx::read(path);
        match book {
            Ok(spreadsheet) => {
                let worksheet: &Worksheet = spreadsheet.get_sheet(&0).unwrap();
                let fecha = read_fecha_by_column_and_row(worksheet, &3, &8);
                assert_eq!(Utc.ymd(2007, 3, 17), fecha.unwrap());
                let bad_fecha = read_fecha_by_column_and_row(worksheet, &4, &8);
                assert!(bad_fecha.is_none());
            }
            _ => panic!("No se pudo leer la hoja de cálculo")
        }
    }

    #[test]
    fn test_read_f64() {
        let path = std::path::Path::new("assets\\libro12.xlsx");
        let book: core::result::Result<Spreadsheet, XlsxError> = reader::xlsx::read(path);
        match book {
            Ok(spreadsheet) => {
                let worksheet: &Worksheet = spreadsheet.get_sheet(&0).unwrap();
                let cap = read_f64(worksheet, "C10");
                assert_eq!(84140.0, cap);
                let bad_cap = read_f64(worksheet, "B2");
                assert_eq!(0.0, bad_cap);
            }
            _ => panic!("No se pudo leer la hoja de cálculo")
        }
    }
    #[test]
    fn test_read_f64_by_column_and_row() {
        let path = std::path::Path::new("assets\\libro12.xlsx");
        let book: core::result::Result<Spreadsheet, XlsxError> = reader::xlsx::read(path);
        match book {
            Ok(spreadsheet) => {
                let worksheet: &Worksheet = spreadsheet.get_sheet(&0).unwrap();
                let cap = read_f64_by_column_and_row(worksheet, &3, &10);
                assert_eq!(84140.0, cap);
                let bad_cap = read_f64_by_column_and_row(worksheet, &2, &2);
                assert_eq!(0.0, bad_cap);
            }
            _ => panic!("No se pudo leer la hoja de cálculo")
        }
    }
}
