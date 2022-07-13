use chrono::prelude::*;
use chrono::Utc;

use super::lib::*;

pub struct Hipoteca {
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
}
impl Hipoteca {
        pub fn new(fecha: Date<Utc>, c_0: f64, i: f64, meses: i32, periodo_1: i32, 
        intervalo_periodos: i32, increm_euribor: f64) -> Self {
            Hipoteca { fecha: fecha, 
                c_0: c_0, 
                i: i, 
                meses: meses, 
                revision_1: periodo_1, 
                intervalo_revisiones: intervalo_periodos, 
                incremento_euribor: increm_euribor,
                cuotas: Vec::<f64>::new(),
                interes_cuotas: Vec::<f64>::new(),
                capital_cuotas: Vec::<f64>::new()
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
            }
            // Ajuste de la ultima cuota por descuadres de redondeo
            if capital_pendiente != 0.0 {
                let x = self.cuotas.last_mut().unwrap();
                *x += capital_pendiente;
                let y = self.capital_cuotas.last_mut().unwrap();
                *y += capital_pendiente;
            } 
        }
        pub fn disp_cuadro_amortizacion(&self) {
            let mut cap_pendiente = self.c_0;
            for _i in 0..self.cuotas.len() {
                cap_pendiente = redondea_dos_decimales(cap_pendiente - self.capital_cuotas[_i]);
                println!("{} {} {} {} {}", _i+1, cap_pendiente, self.cuotas[_i], self.capital_cuotas[_i], self.interes_cuotas[_i]);
            }
        }
}
