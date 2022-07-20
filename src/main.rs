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
    let filename = String::from("C:\\ProgramaHipotecas\\") + &args().last().unwrap();
    println!("{}", filename);
    // reader
    let path = std::path::Path::new(&filename);
    let mut book = reader::xlsx::read(path).unwrap();
    let worksheet = book.get_sheet(&0).unwrap();
    
    let mut h = read_data(worksheet);
    h.tabla_amort_sin_actualizacion = h.calcula_tabla_amort_sin_actualizacion();
    h.tabla_amort_con_actualizacion_euribor = h.calcula_tabla_amort_con_actualizacion_euribor();

    print_txt_files(&h);    

    let _ = book.new_sheet("TablaAmort");
    let worksheet = book.get_sheet_by_name_mut("TablaAmort").unwrap();
    ex_print_cabecera(worksheet);

    ex_print_cuotas(&h, worksheet);


    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
    //wait();
        

    Ok(())
}
fn ex_print_cuotas(h: &Hipoteca, worksheet: &mut Worksheet) {
    let cuotas = &h.tabla_amort_con_actualizacion_euribor.cuotas;
    let mut fila = 2;
    for i in 0..cuotas.len() {
        let cuota = &cuotas[i];
        worksheet.get_cell_by_column_and_row_mut(&1, &fila).set_value(date_to_string(cuota.fecha));
        let _ = worksheet.get_style_by_column_and_row_mut(&1, &fila).get_number_format_mut()
            .set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_DATE_DDMMYYYYSLASH);
        
            worksheet.get_cell_by_column_and_row_mut(&2, &fila).set_value(cuota.i.to_string());
        let _ = worksheet.get_style_by_column_and_row_mut(&2, &fila).get_number_format_mut()
            .set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_PERCENTAGE_00);
        
            worksheet.get_cell_by_column_and_row_mut(&3, &fila).set_value_from_i32(cuota.meses_restantes_antes);
        let _ = worksheet.get_style_by_column_and_row_mut(&3, &fila).get_number_format_mut()
            .set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_NUMBER);

        worksheet.get_cell_by_column_and_row_mut(&4, &fila).set_value(cuota.cap_pendiente_antes.to_string());
        let _ = worksheet.get_style_by_column_and_row_mut(&4, &fila).get_number_format_mut()
                .set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_NUMBER_COMMA_SEPARATED1);
            
        worksheet.get_cell_by_column_and_row_mut(&5, &fila).set_value(cuota.cuota_total.to_string());
        let _ = worksheet.get_style_by_column_and_row_mut(&5, &fila).get_number_format_mut()
                .set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_NUMBER_COMMA_SEPARATED1);
                
        worksheet.get_cell_by_column_and_row_mut(&6, &fila).set_value(cuota.cuota_capital.to_string());
        let _ = worksheet.get_style_by_column_and_row_mut(&6, &fila).get_number_format_mut()
                .set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_NUMBER_COMMA_SEPARATED1);

        worksheet.get_cell_by_column_and_row_mut(&7, &fila).set_value(cuota.cuota_interes.to_string());
        let _ = worksheet.get_style_by_column_and_row_mut(&7, &fila).get_number_format_mut()
                .set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_NUMBER_COMMA_SEPARATED1);

        worksheet.get_cell_by_column_and_row_mut(&8, &fila).set_value(cuota.cap_pendiente_despues().to_string());
        let _ = worksheet.get_style_by_column_and_row_mut(&8, &fila).get_number_format_mut()
                .set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_NUMBER_COMMA_SEPARATED1);
                        
        fila +=1;
    }
}
fn date_to_string(date: Date<Utc>) -> String {
    let fecha = date.day().to_string() + "/" + &date.month().to_string() + &"/" +
        &date.year().to_string();
    fecha
}
fn ex_print_cabecera(worksheet: &mut Worksheet) {
    worksheet.get_column_dimension_by_number_mut(&1).set_auto_width(true);
    worksheet.get_cell_by_column_and_row_mut(&1, &1).set_value("Fecha");
    worksheet.get_column_dimension_by_number_mut(&2).set_auto_width(true);
    worksheet.get_cell_by_column_and_row_mut(&2, &1).set_value("Tipo");
    worksheet.get_column_dimension_by_number_mut(&3).set_auto_width(true);
    worksheet.get_cell_by_column_and_row_mut(&3, &1).set_value("Meses");
    worksheet.get_column_dimension_by_number_mut(&4).set_auto_width(true);
    worksheet.get_cell_by_column_and_row_mut(&4, &1).set_value("Pendiente_Antes");
    worksheet.get_column_dimension_by_number_mut(&5).set_auto_width(true);
    worksheet.get_cell_by_column_and_row_mut(&5, &1).set_value("Cuota_Total");
    worksheet.get_column_dimension_by_number_mut(&6).set_auto_width(true);
    worksheet.get_cell_by_column_and_row_mut(&6, &1).set_value("Cuota_Capital");
    worksheet.get_column_dimension_by_number_mut(&7).set_auto_width(true);
    worksheet.get_cell_by_column_and_row_mut(&7, &1).set_value("Cuota_Intereses");
    worksheet.get_column_dimension_by_number_mut(&8).set_auto_width(true);
    worksheet.get_cell_by_column_and_row_mut(&8, &1).set_value("Pendiente_Después");
}
fn print_txt_files(h: &Hipoteca) {
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
fn read_data(worksheet: &Worksheet) -> Hipoteca {
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
    let h = Hipoteca::new(nombre, fecha,
                capital, tipo, meses, 
                meses_primera_revision, intervalo_revisiones,
                incremento_euribor, i_min, i_max);
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

fn pp() {
//     let mut h = Hipoteca::new(nombre, fecha,
//         capital, tipo, meses, 
//         meses_primera_revision, intervalo_revisiones,
//          incremento_euribor, i_min, i_max);
   
//    h.tabla_amort_sin_actualizacion = h.calcula_tabla_amort_sin_actualizacion();
//    //println!("tabla 1");
//    h.tabla_amort_con_actualizacion_euribor = h.calcula_tabla_amort_con_actualizacion_euribor();
//    //println!("tabla 2");
//    let filename = h.nombre_operacion.clone();
//    let result = h.tabla_amort_sin_actualizacion.print(&filename);
//    if result.is_ok() {
//        println!("El fichero con la tabla de amortización inicial se escribió en {}", h.nombre_operacion.clone()+".txt" );
//    } else {
//        println!("Se produjeron errores al escribir el fichero con la tabla de amortización inicial");
//        println!("{:?}", result);
//    }
//    let filename = h.nombre_operacion.clone() + "_euribor";
//    let result = h.tabla_amort_con_actualizacion_euribor.print(&filename);
//    if result.is_ok() {
//        println!("El fichero con la tabla de amortización con actualizaciones del euribor se escribió en {}", h.nombre_operacion.clone()+"_euribor.txt" );
//    } else {
//        println!("Se produjeron errores al escribir el fichero con la tabla de amortización con actualizaciones del euribor");
//        println!("{:?}", result);
//    }
//    result
}