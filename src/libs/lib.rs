use std::collections::HashMap;


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