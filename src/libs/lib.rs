use chrono::prelude::*;

use super::tabla_amort::TablaAmortizacion;
use super::cuota::Cuota;


pub fn add_one_month(date: Date<Utc>) -> Date<Utc> {
    let agno: i32 = match date.month() {
        12 => date.year()+1,
        _ =>  date.year()
    };
    let mes: u32 = match date.month() {
        12 => 1,
        _ => date.month()+1
    };
    let dia: u32 = match date.month() {
        1 => {
                if date.day() > 28 {
                    if is_leap_year(date.year()) {29} else {28}
                } else { date.day()}
            }, 
        2 | 4 | 6 | 9 | 11 => date.day(),
        3 | 5 | 7 | 8 | 10 | 12 => if date.day() == 31 { 30} else {date.day()}
        _ => panic!("Invalid month")
    };
    Utc.ymd(agno, mes, dia)
}
pub fn add_n_months(date: Date<Utc>, num_meses: i32) -> Date<Utc> {
    let mut fecha:Date<Utc> = date;
    for _i in 1..num_meses+1 {
        fecha = add_one_month(fecha);
    }
    fecha
}
pub fn mes_anterior(year: i32, month: u32) -> (i32, u32) {
    if month>12 {
        panic!("Invalid month!");
    }
    match month {
        1 => (year-1, 12),
        _ => (year, month-1)
    }
}
pub fn is_leap_year(year: i32) -> bool {
    return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
/// Indica el número del último día de un mes
#[allow(dead_code)]
pub fn last_day_of_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => panic!("invalid month: {}" , month),
    }
}

/// Calcula el importe mensual a pagar en un prestamo 
/// con el método de amortización francés (cuotas mensuales iguales)
pub fn importe_cuota_mensual(capital_pendiente:f64, tipo_interes_anual: f64, meses: i32 ) -> f64 {
    let i_mensual: f64 = tipo_interes_anual /12.0;
    let importe_mensualidad: f64 = capital_pendiente * i_mensual / (1.0 - (1.0+i_mensual).powi(-meses));
    redondea_dos_decimales(importe_mensualidad)
}

/// Calcula los intereses a pagar en un mes a partir
/// del capital pendiente y el tipo de interés anual
pub fn intereses_mes(capital_pendiente: f64, tipo_interes_anual: f64) -> f64 {
    redondea_dos_decimales(capital_pendiente*tipo_interes_anual/12.0)
}



pub fn redondea_dos_decimales(valor:f64) -> f64 {
    (valor*100.0).round()/100.0
}
pub fn redondea_cinco_decimales(valor:f64) -> f64 {
    (valor*100000.0).round()/100000.0
}

/// Genera un String con la fecha fecha en formato dd/mm/año a partir de un chrono::Date<Utc>
#[allow(dead_code)]
pub fn date_to_string(date: Date<Utc>) -> String {
    let fecha = date.day().to_string() + "/" + &date.month().to_string() + &"/" +
        &date.year().to_string();
    fecha
}

/// Calcula la tabla de amortización correspondiente a un capital prestado
/// a cierto número de meses con un tipo anual (método francés)
/// 
pub fn calcula_tabla_amortizacion(capital: f64, interes: f64, meses: i32, fecha_primera_cuota: Date<Utc>) ->
    TablaAmortizacion {
    
    let mut tabla_result = TablaAmortizacion::new();

    let mut fecha = fecha_primera_cuota;
    let mut meses_restantes_antes: i32= meses;
    let mut cap_pdte_antes: f64 = capital;
    let cuota_total: f64 = importe_cuota_mensual(cap_pdte_antes, interes, meses);
    for i in 0..meses {
        let cuota_interes: f64 = intereses_mes(cap_pdte_antes, interes);
        let cuota_capital = redondea_dos_decimales(cuota_total - cuota_interes);
        let cuota: Cuota = Cuota::new(fecha, interes, meses_restantes_antes, 
            cap_pdte_antes, cuota_total, cuota_capital, cuota_interes);
        tabla_result.push(cuota);
        fecha = add_one_month(fecha);
        meses_restantes_antes -= 1;
        cap_pdte_antes -= cuota_capital;
    }

    // Ajuste redondeo última cuota
    if cap_pdte_antes != 0.0 {
        let ult_cuota = tabla_result.cuotas.last_mut().unwrap();
        ult_cuota.cuota_total = redondea_dos_decimales(ult_cuota.cuota_total + cap_pdte_antes);
        ult_cuota.cuota_capital = redondea_dos_decimales(ult_cuota.cuota_capital + cap_pdte_antes);
    } 
    tabla_result
}
/// Recibe una tabla de cuotas y una lista de fechas y devuelve una TablaAmortizacion 
/// con las cuotas actualizadas al euribor+incremento. 
pub fn actualiza_euribor(tabla: TablaAmortizacion, 
    fechas: Vec<Date<Utc>>, incremento_euribor: f64, i_min: f64, i_max: f64) -> TablaAmortizacion {
    let tabla_result = TablaAmortizacion::new();

    tabla_result
}
#[cfg(test)]
mod tests {
    use crate::libs::tabla_amort;

    use super::*;
    #[test]
    fn test_calcula_tabla_amortizacion() {
        let capital: f64 = 10000.0;
        let interes: f64 = 0.1;
        let meses: i32 = 60;
        let fecha_primera_cuota = Utc.ymd(2022, 5, 12);
        let tabla_amort = calcula_tabla_amortizacion(capital, interes, meses, fecha_primera_cuota);
        assert_eq!(60, tabla_amort.len());
        tabla_amort.disp();
    }   
    #[test]
    fn test_actualiza_euribor() {
        let fechas = vec![Utc.ymd(2012, 5, 17), Utc.ymd(2012, 11, 17)];
        let tabla = TablaAmortizacion::new();

    }
    #[test]
    fn test_mes_anterior() {
        assert_eq!((2020, 2), mes_anterior(2020,3));
        assert_eq!((2020, 12), mes_anterior(2021,1));
    }
    #[test]
    #[should_panic]
    fn test_mes_anterior_panic() {
        mes_anterior(2020,13);
    }
    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2020));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(2100));
    }
    #[test]
    fn test_last_day_of_month() {
        assert_eq!(31, last_day_of_month(2020, 8));
        assert_eq!(29, last_day_of_month(2000, 2));
        assert_eq!(28, last_day_of_month(2100, 2));
    }
    #[test]
    #[should_panic]
    fn test_last_day_of_month_panic() {
        last_day_of_month(2020, 25);
    }

    #[test]
    fn test_add_one_month() {
        let dt = Utc.ymd(2022, 1, 1);
        let dt2 = add_one_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(1, dt2.day());

        let dt = Utc.ymd(2022, 2, 15);
        let dt2 = add_one_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(3, dt2.month());
        assert_eq!(15, dt2.day());

        let dt = Utc.ymd(2022, 12, 10);
        let dt2 = add_one_month(dt);
        assert_eq!(2023, dt2.year());
        assert_eq!(1, dt2.month());
        assert_eq!(10, dt2.day());

        let dt = Utc.ymd(2022, 1, 29);
        let dt2 = add_one_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(28, dt2.day());

        let dt = Utc.ymd(2022, 1, 30);
        let dt2 = add_one_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(28, dt2.day());

        let dt = Utc.ymd(2022, 1, 31);
        let dt2 = add_one_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(28, dt2.day());

        let dt = Utc.ymd(2000, 1, 29);
        let dt2 = add_one_month(dt);
        assert_eq!(2000, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(29, dt2.day());

        let dt = Utc.ymd(2022, 1, 30);
        let dt2 = add_one_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(28, dt2.day());

        let dt = Utc.ymd(2022, 1, 31);
        let dt2 = add_one_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(28, dt2.day());
    
        let dt = Utc.ymd(2022, 3, 31);
        let dt2 = add_one_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(4, dt2.month());
        assert_eq!(30, dt2.day());

        /*let dt = Utc.ymd(2022, 5, 31);
        let dt2 = add_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(6, dt2.month());
        assert_eq!(30, dt2.day());

        let dt = Utc.ymd(2022, 7, 31);
        let dt2 = add_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(8, dt2.month());
        assert_eq!(31, dt2.day());

        let dt = Utc.ymd(2022, 8, 31);
        let dt2 = add_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(9, dt2.month());
        assert_eq!(30, dt2.day());

        let dt = Utc.ymd(2022, 10, 31);
        let dt2 = add_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(11, dt2.month());
        assert_eq!(30, dt2.day());

        let dt = Utc.ymd(2022, 12, 31);
        let dt2 = add_month(dt);
        assert_eq!(2023, dt2.year());
        assert_eq!(1, dt2.month());
        assert_eq!(31, dt2.day());
        */
    }
    #[test]
    fn test_add_n_months() {
        let fecha = Utc.ymd(2004, 3, 17);
        let meses = 300;
        assert_eq!(Utc.ymd(2029, 3, 17), add_n_months(fecha, meses));
    }
   
    #[test]
    fn test_redondea_dos_decimales() {
        let x: f64 = 1324.7856;
        assert_eq!(1324.79, redondea_dos_decimales(x));
        let x:f64 = 1324.7816;
        assert_eq!(1324.78, redondea_dos_decimales(x));
        let x:f64 = 1324.785;
        assert_eq!(1324.79, redondea_dos_decimales(x));
    }

}