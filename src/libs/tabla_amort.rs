use super::{cuota::Cuota};
//use chrono::prelude::*;
use std::fs::File;
use std::io::{Write};


pub struct TablaAmortizacion {
    pub cuotas: Vec<Cuota>,
}

impl TablaAmortizacion {
    pub fn new() -> Self {
        TablaAmortizacion { cuotas: Vec::<Cuota>::new() }
    }
    pub fn len(&self) -> usize {
        self.cuotas.len()
    }
    pub fn push(&mut self, cuota: Cuota) {
        self.cuotas.push(cuota);
    }
    #[allow(dead_code)]
    pub fn disp(&self) {
        for i in 0..self.len() {
            let cuota = &self.cuotas[i];
            cuota.disp();            
        }
    }
    pub fn print(&self, nombre: &str) -> std::io::Result<()> {
        let filename = String::from(nombre) + ".txt";
        //println!("{}", filename);
        let file = File::create(filename)?;
        // Escribir la línea de cabeceras
        writeln!(&file, "{}; {}; {}; {}; {}; {}; {}; {}", "Fecha","i", "Meses","Pendiente_antes",
             "Cuota", "Capital", "Intereses", "Pendiente_despues")?;
        for i in 0..self.len() {
            writeln!(&file, "{}" , self.cuotas[i].to_csv_string())?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use std::io::Result;
    use crate::libs::hipoteca::Hipoteca;

    #[test]
    fn test_new() {
        let tabla = TablaAmortizacion::new();
        assert_eq!(0, tabla.cuotas.len());
    }
    #[test]
    fn test_len() {
    }
    #[test]
    fn test_push() {
        let mut tabla = TablaAmortizacion::new();
        let cuota = Cuota::new(
            Utc.ymd(2004, 4, 17),0.04,
            300, 84140.0, 444.12, 
            163.65, 280.47); 
        tabla.push(cuota);
        assert_eq!(1, tabla.len());
    }
    #[test]
    fn test_disp() {
        let mut tabla = TablaAmortizacion::new();
        for _i in 1..10 {
            let cuota = Cuota::new(
                Utc.ymd(2004, 4, 17),0.04,
                300, 84140.0, 444.12, 
                163.65, 280.47);
            tabla.push(cuota);
        }
        tabla.disp();    
    }
    #[test]
    fn test_print() -> Result<()> {
        let nombre = String::from("h1");
        let fecha = Utc.ymd(2004,3,17);
        let mut h1= Hipoteca::new(nombre, fecha, 84140.0, 0.04,
            300,6,12,0.01, 
            0.04, 0.12, Utc.ymd(2018, 5, 17),
            Utc.ymd(2022, 8, 5));        
        h1.tabla_amort_sin_actualizacion = h1.calcula_tabla_amort_sin_actualizacion();
        h1.tabla_amort_con_actualizacion_euribor = h1.calcula_tabla_amort_con_actualizacion_euribor();
        h1.tabla_amort_sin_actualizacion.print(&h1.nombre_operacion.clone())?;
        let filename = h1.nombre_operacion.clone()+"_euribor";
        h1.tabla_amort_con_actualizacion_euribor.print(&filename)?;  
        Ok(())  
    }
}