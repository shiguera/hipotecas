use super::{cuota::Cuota, hipoteca::Hipoteca, lib};
use chrono::prelude::*;


pub struct Tabla_Amortizacion {
    pub cuotas: Vec<Cuota>,
}

impl Tabla_Amortizacion {
    pub fn new() -> Self {
        Tabla_Amortizacion { cuotas: Vec::<Cuota>::new() }
    }
    pub fn len(&self) -> usize {
        self.cuotas.len()
    }
    pub fn push(&mut self, cuota: Cuota) {
        self.cuotas.push(cuota);
    }
    pub fn disp(&self) {
        for i in 0..self.len() {
            let cuota = &self.cuotas[i];
            cuota.disp();            
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tabla = Tabla_Amortizacion::new();
        assert_eq!(0, tabla.cuotas.len());
    }
    #[test]
    fn test_len() {
    }
    #[test]
    fn test_push() {
        let mut tabla = Tabla_Amortizacion::new();
        let cuota = Cuota::new(
            Utc.ymd(2004, 4, 17),0.04,
            300, 84140.0, 444.12, 
            163.65, 280.47); 
        tabla.push(cuota);
        assert_eq!(1, tabla.len());
    }
    #[test]
    fn test_disp() {
        let mut tabla = Tabla_Amortizacion::new();
        for i in 1..10 {
            let cuota = Cuota::new(
                Utc.ymd(2004, 4, 17),0.04,
                300, 84140.0, 444.12, 
                163.65, 280.47);
            tabla.push(cuota);
        }
        tabla.disp();    
    }
}