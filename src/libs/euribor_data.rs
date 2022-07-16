use std::collections::HashMap;
use chrono::prelude::*;
pub struct EuriborData {
    tabla: HashMap<usize, Vec<f64>>,
}

impl EuriborData {
    pub fn new() -> Self {
        EuriborData { tabla: Self::read_euribor_table() }
    }
    fn read_euribor_table() -> HashMap<usize, Vec<f64>> {
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
    pub fn last_year(&self) -> usize {
        let last_year: usize = *self.tabla.keys().into_iter().max().unwrap();
        last_year
    }
    /// Devuelve el euribor correspondiente a un mes.
    /// Si el mes es posterior al último de los almacenados en la tabla de datos,
    /// devuelve el correspondiente al mes 12 del último año de la tabla de datos
    pub fn euribor_mes(&self, tabla: &HashMap<usize, Vec<f64>>, mes:usize, agno: usize) -> f64 {
        let mut agno_posible = agno;
        let mut mes_posible = mes;
        let agno_max = self.last_year();
        if agno >  agno_max {
            agno_posible = agno_max;
            mes_posible = 12;
        }
        tabla.get(&agno_posible).unwrap()[mes_posible-1]
    }
    /// Devuelve el euribor correspondiente al cierre del mes anterior
    pub fn euribor_fecha(&self, tabla: &HashMap<usize, Vec<f64>>, fecha: Date<Utc>) -> f64 {
        let mes = usize::try_from(fecha.month()).ok().unwrap();
        let agno = usize::try_from(fecha.year()).ok().unwrap();
        self.euribor_mes(tabla, mes, agno)
    }
    
}

#[cfg(test)]
mod tests {
    use super::EuriborData;

    #[test]
    fn test_new() {
        let ed = EuriborData::new();
    }
    #[test]
    fn test_last_year() {
        let ed = EuriborData::new();
        assert_eq!(2021, ed.last_year());
    }
    #[test]
    fn test_euribor_mes() {
        let ed = EuriborData::new();
        let mes:usize = 6;
        let agno: usize = 2021;
        assert_eq!(-0.484, ed.euribor_mes(&ed.tabla, mes, agno));
        let mes:usize = 4;
        let agno: usize = 2020;
        assert_eq!(-0.108, ed.euribor_mes(&ed.tabla, mes, agno));
        let mes:usize = 4;
        let agno: usize = 2022;
        assert_eq!(-0.502, ed.euribor_mes(&ed.tabla, mes, agno));
    }
    #[test]
    fn test_euribor_fecha() {
        let ed = EuriborData::new();
        let mes:usize = 6;
        let agno: usize = 2021;
        assert_eq!(-0.484, ed.euribor_mes(&ed.tabla, mes, agno));
        let mes:usize = 4;
        let agno: usize = 2020;
        assert_eq!(-0.108, ed.euribor_mes(&ed.tabla, mes, agno));
        let mes:usize = 4;
        let agno: usize = 2022;
        assert_eq!(-0.502, ed.euribor_mes(&ed.tabla, mes, agno));
    }

}