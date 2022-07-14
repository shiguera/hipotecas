use chrono::prelude::*;
use chrono::Utc;

use super::lib::*;

pub struct Hipoteca {
    nombre_operacion: String, // Usado para los ficheros que se creen
    fecha: Date<Utc>, // Fecha inicial de la hipoteca
    c_0: f64, // Capital inicial prestado
    i: f64, // Tipo de interés nominal anual
    meses: i32, // Meses para amortizar la hipoteca
    revision_1: i32, // Meses de aplicación del tipo de interes nominal inicial
    intervalo_revisiones: i32, // Meses entre actualización tipos interés
    incremento_euribor: f64, // Incremento aplicado al euribor en las revisiones
    cuotas: Vec<f64>, // Importe de las cuotas mensuales
    interes_cuotas: Vec<f64>, // Importe de la parte de intereses de las cuotas
    capital_cuotas: Vec<f64>, // Importe de la parte de amortización del capital de las cuotas
    capital_pendiente: Vec<f64> // Capital pendiente de amortización en cada periodo
}
impl Hipoteca {
        pub fn new(nombre: String, fecha: Date<Utc>, c_0: f64, i: f64, meses: i32, periodo_1: i32, 
        intervalo_periodos: i32, increm_euribor: f64) -> Self {
            Hipoteca { 
                nombre_operacion: nombre,
                fecha: fecha, 
                c_0: c_0, 
                i: i, 
                meses: meses, 
                revision_1: periodo_1, 
                intervalo_revisiones: intervalo_periodos, 
                incremento_euribor: increm_euribor,
                cuotas: Vec::<f64>::new(),
                interes_cuotas: Vec::<f64>::new(),
                capital_cuotas: Vec::<f64>::new(),
                capital_pendiente: Vec::<f64>::new()
            }

        }
        pub fn mensualidad(&self) -> f64 {
            let i_mensual: f64 = self.i /12.0;
            let a: f64 = self.c_0 * i_mensual / (1.0 - (1.0+i_mensual).powi(-self.meses));
            redondea_dos_decimales(a)
        }
        pub fn calcula_cuotas(&mut self)  {
            let a = self.mensualidad();
            let mut capital_pendiente: f64 = self.c_0;
            for _i in 1..self.meses+1 {
                self.cuotas.push(a);
                let interes_cuota = redondea_dos_decimales(capital_pendiente*self.i/12.0);
                self.interes_cuotas.push(interes_cuota);
                let capital_cuota = redondea_dos_decimales(a-interes_cuota);
                self.capital_cuotas.push(capital_cuota);
                capital_pendiente = redondea_dos_decimales(capital_pendiente - capital_cuota);
                self.capital_pendiente.push(capital_pendiente);
            }
            // Ajuste de la ultima cuota por descuadres de redondeo
            if capital_pendiente != 0.0 {
                let x = self.cuotas.last_mut().unwrap();
                *x += capital_pendiente;
                let y = self.capital_cuotas.last_mut().unwrap();
                *y += capital_pendiente;
                let z = self.capital_pendiente.last_mut().unwrap();
                *z = 0.0;
            } 
        }
        pub fn disp_cuadro_amortizacion(&self) {
            for _i in 0..self.cuotas.len() {
                println!("{} {} {} {} {}", _i+1, self.capital_pendiente[_i], 
                    self.cuotas[_i], self.capital_cuotas[_i], self.interes_cuotas[_i]);
            }
        }
}

mod tests {
    use super::*;

    #[test]
    fn test_hipoteca() {
        let nombre = String::from("Prueba");
        let mut h1= Hipoteca::new(nombre, Utc.ymd(2004,3,17), 84140.0, 0.04,300,6,12,0.01);
        h1.calcula_cuotas();
        assert_eq!(444.12, h1.cuotas[0]);
        assert_eq!(163.65, h1.capital_cuotas[0]);
        assert_eq!(280.47, h1.interes_cuotas[0]);
        assert_eq!(83976.35, h1.capital_pendiente[0]);
        
        assert_eq!(444.12, h1.cuotas[298]);
        assert_eq!(441.17, h1.capital_cuotas[298]);
        assert_eq!(2.95, h1.interes_cuotas[298]);
        assert_eq!(443.64, h1.capital_pendiente[298]);
        
        assert_eq!(445.12, h1.cuotas[299]);
        assert_eq!(443.64, h1.capital_cuotas[299]);
        assert_eq!(1.48, h1.interes_cuotas[299]);
        assert_eq!(0.0, h1.capital_pendiente[299]);
        

    }
}
