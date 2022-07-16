use chrono::prelude::*;

use std::collections::HashMap;

pub fn add_month(date: Date<Utc>) -> Date<Utc> {
    let dt:Date<Utc>;
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
pub fn last_day_of_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => panic!("invalid month: {}" , month),
    }
}

pub fn mensualidad(c_0: f64, i_anual: f64, meses: i32) -> f64 {
    let i_mensual: f64 = i_anual/12.0;
    let a: f64 = c_0 * i_mensual / (1.0 - (1.0+i_mensual).powi(-meses));
    redondea_dos_decimales(a)
}
pub fn interes_periodo(c_0: f64, i: f64) -> f64 {
    redondea_dos_decimales(c_0*i)
}
pub fn capital_periodo(a: f64, interes: f64) -> f64 {
    redondea_dos_decimales(a-interes)
}
pub fn redondea_dos_decimales(valor:f64) -> f64 {
    (valor*100.0).round()/100.0
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_add_month() {
        let dt = Utc.ymd(2022, 1, 1);
        let dt2 = add_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(1, dt2.day());

        let dt = Utc.ymd(2022, 2, 15);
        let dt2 = add_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(3, dt2.month());
        assert_eq!(15, dt2.day());

        let dt = Utc.ymd(2022, 12, 10);
        let dt2 = add_month(dt);
        assert_eq!(2023, dt2.year());
        assert_eq!(1, dt2.month());
        assert_eq!(10, dt2.day());

        let dt = Utc.ymd(2022, 1, 29);
        let dt2 = add_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(28, dt2.day());

        let dt = Utc.ymd(2022, 1, 30);
        let dt2 = add_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(28, dt2.day());

        let dt = Utc.ymd(2022, 1, 31);
        let dt2 = add_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(28, dt2.day());

        let dt = Utc.ymd(2000, 1, 29);
        let dt2 = add_month(dt);
        assert_eq!(2000, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(29, dt2.day());

        let dt = Utc.ymd(2022, 1, 30);
        let dt2 = add_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(28, dt2.day());

        let dt = Utc.ymd(2022, 1, 31);
        let dt2 = add_month(dt);
        assert_eq!(2022, dt2.year());
        assert_eq!(2, dt2.month());
        assert_eq!(28, dt2.day());
    
        let dt = Utc.ymd(2022, 3, 31);
        let dt2 = add_month(dt);
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
    fn test_interes_periodo() {
        let int:f64 = interes_periodo(84140.0, 0.04/12.0);
        assert_eq!(280.47, int);
    }
    #[test]
    fn test_capital_periodo() {
        assert_eq!(163.65, capital_periodo(444.12, 280.47));
    }
    #[test]
    fn test_mensualidad() {
        let c_0: f64 = 84140.0;
        let i_anual: f64 = 0.04;
        let meses: i32 = 300;
        assert_eq!(444.12, mensualidad(c_0, i_anual, meses));
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