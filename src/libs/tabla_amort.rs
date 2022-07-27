use super::euribor_data::EuriborData;
use super::{cuota::Cuota};
use super::lib::*;
use chrono::prelude::*;
use std::fs::File;
use std::io::{Write};


pub struct TablaAmortizacion {
    pub cuotas: Vec<Cuota>,
}
impl Clone for TablaAmortizacion {
    fn clone(&self) -> Self {
        let mut tabla_result = TablaAmortizacion::new();
        self.cuotas.iter().for_each(|x| tabla_result.push(x.clone()));
        tabla_result
    }
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
    pub fn push_tabla(&mut self, tabla: &TablaAmortizacion) {
        for i in 0..tabla.len() {
            self.push(tabla.cuotas[i].clone());
        }
    }
    pub fn actualiza_euribor(&mut self, fecha: Date<Utc>, incremento: f64, i_min: f64, i_max: f64) 
        -> TablaAmortizacion {
        let mut tabla_actualizada = TablaAmortizacion::new();
        let ed = EuriborData::new();
        let tipo_actualizado = ed.actualiza_euribor(fecha, incremento, i_min, i_max);
        let tabla_antes = self.tabla_hasta_fecha_excluida(fecha);
        let mut tabla_despues = self.tabla_desde_fecha_incluida(fecha);
        if tabla_despues.len() == 0 {
            return self.clone()
        }
        let cap_pdte = tabla_despues.cuotas[0].cap_pendiente_antes;
        let meses = tabla_despues.cuotas[0].meses_restantes_antes;
        let fecha_primera_cuota = tabla_despues.cuotas[0].fecha;
        tabla_despues = calcula_tabla_amortizacion(cap_pdte, tipo_actualizado, meses, fecha_primera_cuota);
        tabla_actualizada.push_tabla(&tabla_antes);
        tabla_actualizada.push_tabla(&tabla_despues);
        tabla_actualizada 
    }
    pub fn tabla_hasta_fecha_incluida(&self, fecha: Date<Utc>) -> TablaAmortizacion {
        let mut tabla_result = TablaAmortizacion::new();
        self.cuotas.iter()
            .filter(|x| x.fecha <= fecha)
            .for_each(|x| tabla_result.push(x.clone()));
        tabla_result
    }
    pub fn tabla_hasta_fecha_excluida(&self, fecha: Date<Utc>) -> TablaAmortizacion {
        let mut tabla_result = TablaAmortizacion::new();
        self.cuotas.iter()
            .filter(|x| x.fecha < fecha)
            .for_each(|x| tabla_result.push(x.clone()));
        tabla_result
    }
    pub fn tabla_desde_fecha_incluida(&self, fecha: Date<Utc>) -> TablaAmortizacion {
        let mut tabla_result = TablaAmortizacion::new();
        self.cuotas.iter()
            .filter(|x| x.fecha >= fecha)
            .for_each(|x| tabla_result.push(x.clone()));
        tabla_result
    }
    pub fn tabla_desde_fecha_excluida(&self, fecha: Date<Utc>) -> TablaAmortizacion {
        let mut tabla_result = TablaAmortizacion::new();
        self.cuotas.iter()
            .filter(|x| x.fecha > fecha)
            .for_each(|x| tabla_result.push(x.clone()));
        tabla_result
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

    fn create_sample_table() -> TablaAmortizacion {
        let nombre = String::from("Prueba");
        let fecha = Utc.ymd(2004,3,17);
        let mut h1= Hipoteca::new(nombre, fecha, 
            Utc.ymd(2004, 4, 17), 10000.0, 0.04,
            5,1,2,0.00, 
            0.00, 0.12, Some(Utc.ymd(2018, 5, 17)),
            Some(Utc.ymd(2022, 8, 5)));
        h1.tabla_amort_sin_actualizacion

    }

    #[test]
    fn test_push_tabla() {
        let mut t1 = create_sample_table();
        let t2 = create_sample_table();
        t1.push_tabla(&t2);
        assert_eq!(t2.len()*2, t1.len());
    }

    #[test]
    fn test_clone() {
        let t = create_sample_table();
        let t2 = t.clone();
        assert_eq!(t.len(), t2.len());
        for i in 0..t2.len() {
            assert_eq!(t.cuotas[i].fecha, t2.cuotas[i].fecha);
            assert_eq!(t.cuotas[i].cap_pendiente_antes, t2.cuotas[i].cap_pendiente_antes);
            assert_eq!(t.cuotas[i].cuota_total, t2.cuotas[i].cuota_total);
        }        
    }
    #[test]
    fn test_tabla_desde_fecha_incluida() {
        let tabla = create_sample_table();
        let tabla_result = tabla.tabla_desde_fecha_incluida(Utc.ymd(2004, 7, 15));
        assert_eq!(2, tabla_result.len());
        let tabla_result = tabla.tabla_desde_fecha_incluida(Utc.ymd(2004, 7, 17));
        assert_eq!(2, tabla_result.len());
        let tabla_result = tabla.tabla_desde_fecha_incluida(Utc.ymd(2005, 7, 15));
        assert_eq!(0, tabla_result.len());
        let tabla_result = tabla.tabla_desde_fecha_incluida(Utc.ymd(2001, 7, 15));
        assert_eq!(5, tabla_result.len());
    }
    #[test]
    fn test_tabla_desde_fecha_excluida() {
        let tabla = create_sample_table();
        let tabla_result = tabla.tabla_desde_fecha_excluida(Utc.ymd(2004, 7, 15));
        assert_eq!(2, tabla_result.len());
        let tabla_result = tabla.tabla_desde_fecha_excluida(Utc.ymd(2004, 7, 17));
        assert_eq!(1, tabla_result.len());
        let tabla_result = tabla.tabla_desde_fecha_excluida(Utc.ymd(2005, 7, 15));
        assert_eq!(0, tabla_result.len());
        let tabla_result = tabla.tabla_desde_fecha_excluida(Utc.ymd(2001, 7, 15));
        assert_eq!(5, tabla_result.len());   
    }
    #[test]
    fn test_tabla_hasta_fecha_incluida() {
        let tabla = create_sample_table();
        let tabla_result = tabla.tabla_hasta_fecha_incluida(Utc.ymd(2004, 7, 15));
        assert_eq!(3, tabla_result.len());
        let tabla_result = tabla.tabla_hasta_fecha_incluida(Utc.ymd(2004, 7, 17));
        assert_eq!(4, tabla_result.len());
        let tabla_result = tabla.tabla_hasta_fecha_incluida(Utc.ymd(2005, 7, 15));
        assert_eq!(5, tabla_result.len());
        let tabla_result = tabla.tabla_hasta_fecha_incluida(Utc.ymd(2004, 3, 17));
        assert_eq!(0, tabla_result.len());           
    }
    #[test]
    fn test_tabla_hasta_fecha_excluida() {
        let tabla = create_sample_table();
        tabla.disp();
        let tabla_result = tabla.tabla_hasta_fecha_excluida(Utc.ymd(2004, 7, 15));
        assert_eq!(3, tabla_result.len());
        let tabla_result = tabla.tabla_hasta_fecha_excluida(Utc.ymd(2004, 7, 17));
        assert_eq!(3, tabla_result.len());
        let tabla_result = tabla.tabla_hasta_fecha_excluida(Utc.ymd(2005, 7, 15));
        assert_eq!(5, tabla_result.len());
        let tabla_result = tabla.tabla_hasta_fecha_excluida(Utc.ymd(2004, 3, 17));
        assert_eq!(0, tabla_result.len());           
    }
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
        let mut h1= Hipoteca::new(nombre, fecha,
            Utc.ymd(2004, 4, 17), 84140.0, 0.04,
            300,6,12,0.01, 
            0.04, 0.12, Some(Utc.ymd(2018, 5, 17)),
            Some(Utc.ymd(2022, 8, 5)));        
        h1.tabla_amort_sin_actualizacion = h1.calcula_tabla_amort_sin_actualizacion();
        h1.tabla_amort_con_actualizacion_euribor = h1.calcula_tabla_amort_con_actualizacion_euribor();
        h1.tabla_amort_sin_actualizacion.print(&h1.nombre_operacion.clone())?;
        let filename = h1.nombre_operacion.clone()+"_euribor";
        h1.tabla_amort_con_actualizacion_euribor.print(&filename)?;  
        Ok(())  
    }

    #[test]
    fn test_actualiza_euribor() {
        let nombre = String::from("Prueba");
        let fecha = Utc.ymd(2004,3,17);
        let mut h1= Hipoteca::new(nombre, fecha, 
            Utc.ymd(2004, 4, 17), 10000.0, 0.04,
            5,1,2,0.00, 
            0.00, 0.12, Some(Utc.ymd(2018, 5, 17)),
            Some(Utc.ymd(2022, 8, 5)));
        h1.tabla_amort_sin_actualizacion.disp();
        h1.tabla_amort_con_actualizacion_euribor.disp();
        let tabla_actualizada = h1.tabla_amort_sin_actualizacion.actualiza_euribor(
            Utc.ymd(2004, 5, 17), 0.0, 0.0, 0.12);
        
    }
}