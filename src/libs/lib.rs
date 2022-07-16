use chrono::prelude::*;

use std::collections::HashMap;

pub fn add_month(date: Date<Utc>) -> Date<Utc> {
    let dt:Date<Utc>;
    if date.month()<12 {
        dt = Utc.ymd(date.year(), date.month()+1, date.day());
    } else {
        dt = Utc.ymd(date.year()+1, 1, date.day());
    }
    dt
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
pub fn euribor(tabla: &HashMap<usize, Vec<f64>>, mes:usize, agno: usize) -> f64 {
    tabla.get(&agno).unwrap()[mes-1]
}
pub fn read_euribor_table() -> HashMap<usize, Vec<f64>> {
    let mut eu = HashMap::new();
    // 2004 
    let v:Vec<f64> = vec![2.216, 2.163, 2.055, 2.163, 2.297, 2.404, 2.361, 2.302, 2.377, 2.316, 2.328, 2.301];
    let agno: usize = 2004;
    eu.insert(agno, v);
    // 2005 
    let v:Vec<f64> = vec![2.312, 2.310, 2.335, 2.265, 2.193, 2.103, 2.168, 2.223, 2.220, 2.414, 2.684, 2.783];
    let agno: usize = 2005;
    eu.insert(agno, v);
    // 2006
    let v:Vec<f64> = vec![2.833, 2.914, 3.100, 2.221, 2.308, 3.401, 3.547, 3.720, 3.715, 3.799, 3.864, 3.921]; 
    let agno: usize = 2006;
    eu.insert(agno, v);
    // 2007
    let v:Vec<f64> = vec![4.064, 4.094, 4.106, 4.253, 4.373, 4.505, 4.564, 4.666, 4.725, 4.647, 4.607, 4.793]; 
    let agno: usize = 2007;
    eu.insert(agno, v);
    // 2008
    let v:Vec<f64> = vec![4.498, 4.349, 4.590, 4.820, 4.994, 5.361, 5.393, 5.323, 5.384, 5.248, 4.350, 3.452]; 
    let agno: usize = 2008;
    eu.insert(agno, v);
    // 2009
    let v:Vec<f64> = vec![2.622, 2.135, 1.909, 1.771, 1.644, 1.610, 1.412, 1.334, 1.261, 1.243, 1.231, 1.242]; 
    let agno: usize = 2009;
    eu.insert(agno, v);
    // 2010
    let v:Vec<f64> = vec![1.232, 1.225, 1.215, 1.225, 1.249, 1.281, 1.373, 1.421, 1.420, 1.495, 1.541, 1.526]; 
    let agno: usize = 2010;
    eu.insert(agno, v);
    // 2011
    let v:Vec<f64> = vec![1.550, 1.714, 1.924, 2.086, 2.147, 2.144, 2.183, 2.097, 2.067, 2.110, 2.044, 2.004]; 
    let agno: usize = 2011;
    eu.insert(agno, v);
    // 2012
    let v:Vec<f64> = vec![1.837, 1.678, 1.499, 1.368, 1.266, 1.219, 1.061, 0.877, 0.740, 0.650, 0.588, 0.549];
    let agno: usize = 2012;
    eu.insert(agno, v);
    // 2013
    let v:Vec<f64> = vec![0.575, 0.594, 0.545, 0.528, 0.484, 0.507, 0.525, 0.542, 0.543, 0.541, 0.506, 0.543]; 
    let agno: usize = 2013;
    eu.insert(agno, v);
    // 2014
    let v:Vec<f64> = vec![0.562, 0.549, 0.577, 0.604, 0.592, 0.513, 0.488, 0.469, 0.362, 0.338, 0.335, 0.329];
    let agno: usize = 2014;
    eu.insert(agno, v);
    // 2015
    let v:Vec<f64> = vec![0.298, 0.255, 0.212, 0.180, 0.165, 0.163, 0.167, 0.161, 0.154, 0.128, 0.079, 0.059];
    let agno: usize = 2015;
    eu.insert(agno, v);
    // 2016
    let v:Vec<f64> = vec![0.042, -0.008, -0.012, -0.010, -0.013, -0.028, -0.056, -0.048, -0.057, -0.069, -0.074, -0.080];
    let agno: usize = 2016;
    eu.insert(agno, v);
    // 2017
    let v:Vec<f64> = vec![-0.095, -0.106, -0.110, -0.119, -0.127, -0.149, -0.154, -0.156, -0.168, -0.180, -0.189, -0.190];
    let agno: usize = 2017;
    eu.insert(agno, v);
    // 2018
    let v:Vec<f64> = vec![-0.189, -0.191, -0.191, -0.190, -0.188, -0.181, -0.180, -0.169, -0.166, -0.154, -0.147, -0.129]; 
    let agno: usize = 2018;
    eu.insert(agno, v);
    // 2019
    let v:Vec<f64> = vec![-0.116, -0.108, -0.109, -0.112, -0.134, -0.190, -0.283, -0.356, -0.339, -0.304, -0.272, -0.261]; 
    let agno: usize = 2019;
    eu.insert(agno, v);    
    // 2020
    let v:Vec<f64> = vec![-0.253,-0.288,-0.266,-0.108,-0.081,-0.147,-0.279,-0.359,-0.415,-0.466,-0.481,-0.497];
    let agno: usize = 2020;
    eu.insert(agno, v);
    // 2021
    let v:Vec<f64> = vec![-0.505,-0.501,-0.487,-0.484,-0.481,-0.484,-0.491,-0.498,-0.492,-0.477,-0.487,-0.502];
    let agno: usize = 2021;
    eu.insert(agno, v);
    eu
} 

#[cfg(test)]
mod tests {
    use super::*;
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
    #[test]
    fn test_euribor() {
        let eu = read_euribor_table();
        let mes:usize = 6;
        let agno: usize = 2021;
        assert_eq!(-0.484, euribor(&eu, mes, agno));
        let mes:usize = 4;
        let agno: usize = 2020;
        assert_eq!(-0.108, euribor(&eu, mes, agno));
    }
}