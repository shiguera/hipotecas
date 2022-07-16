use chrono::prelude::*;
use chrono::Utc;

use super::cuota::Cuota;
use super::lib;
use super::lib::*;
use super::tabla_amort::Tabla_Amortizacion;

pub struct Hipoteca {
    // Usado para los nombres de los ficheros que se creen
    pub nombre_operacion: String, 
    // Fecha inicial de la hipoteca establecida en la escritura
    pub fecha: Date<Utc>, 
    // Capital inicial prestado, según establece la escritura
    pub c_0: f64, 
    // Tipo de interés nominal anual establecido por la escritura
    pub i: f64, 
    // Meses para amortizar la hipoteca
    pub meses: i32, 
    // Meses de aplicación del tipo de interes nominal inicial, antes 
    // de la primera actualización
    pub meses_hasta_primera_revision: i32, 
    // Meses entre actualizaciones de tipos interés
    pub intervalo_revisiones: i32,
    // Incremento aplicado sobre el euribor en las revisiones 
    pub incremento_euribor: f64, 
    // Tipo mínimo establecido en las cláusulas de la hipoteca
    pub i_min: f64, 
    // Tipo máximo establecido en las cláusulas de la hipoteca
    pub i_max: f64, 
    // Tabla de amortización completa, pero con todas
    // las cuotas calculadas con el interés inicial i
    pub tabla_amort_inicial: Tabla_Amortizacion,
    pub tabla_amort_con_actualizaciones_euribor: Tabla_Amortizacion, 
}
impl Hipoteca {
        pub fn new(nombre: String, fecha: Date<Utc>, c_0: f64, 
            i: f64, meses: i32, periodo_1: i32, 
            intervalo_revisiones: i32, incremento_euribor: f64, 
            i_min: f64, i_max: f64) -> Self {
            Hipoteca { 
                nombre_operacion: nombre,
                fecha: fecha, 
                c_0: c_0, 
                i: i,  
                meses: meses, 
                meses_hasta_primera_revision: periodo_1, 
                intervalo_revisiones,  
                incremento_euribor, 
                i_min, 
                i_max, 
                tabla_amort_inicial: Tabla_Amortizacion::new(),
                tabla_amort_con_actualizaciones_euribor: Tabla_Amortizacion::new(),
            }
        }
        pub fn mensualidad(&self) -> f64 {
            let i_mensual: f64 = self.i /12.0;
            let a: f64 = self.c_0 * i_mensual / (1.0 - (1.0+i_mensual).powi(-self.meses));
            redondea_dos_decimales(a)
        }
        pub fn calcula_tabla_amort_inicial(&mut self) {
            let a = self.mensualidad();
            let mut capital_pendiente_antes: f64 = self.c_0;
            let mut fecha_cuota: Date<Utc> = lib::add_month(self.fecha);            
            let mut meses_restantes_antes = self.meses;
            for i in 0..self.meses {
                let cuota_interes = redondea_dos_decimales(capital_pendiente_antes*self.i/12.0);
                let cuota_capital = redondea_dos_decimales(a-cuota_interes);
                let cuota: Cuota = Cuota::new(fecha_cuota, self.i, meses_restantes_antes,
                    capital_pendiente_antes,a, cuota_capital, cuota_interes);
                self.tabla_amort_inicial.push(cuota);
                capital_pendiente_antes = redondea_dos_decimales(capital_pendiente_antes - cuota_capital);
                fecha_cuota = lib::add_month(fecha_cuota);
                meses_restantes_antes -= 1;
            }
            // Ajuste de la ultima cuota por descuadres de redondeo
            if capital_pendiente_antes != 0.0 {
                let ult_cuota = self.tabla_amort_inicial.cuotas.last_mut().unwrap();
                ult_cuota.cap_pendiente_antes += capital_pendiente_antes;
                ult_cuota.cuota_total += capital_pendiente_antes;
                ult_cuota.cuota_capital += capital_pendiente_antes;
            } 
            
        }
        fn calcula_amort_primer_periodo(&mut self) {
            let a = self.mensualidad();
            let mut capital_pendiente: f64 = self.c_0;
            let mut fecha_cuota: Date<Utc> = lib::add_month(self.fecha);            
            let mut meses_restantes_antes = self.meses;
            for i in 0..self.meses_hasta_primera_revision {
                let cuota_interes = redondea_dos_decimales(capital_pendiente*self.i/12.0);
                let cuota_capital = redondea_dos_decimales(a-cuota_interes);
                let cuota: Cuota = Cuota::new(fecha_cuota, self.i, meses_restantes_antes,
                    capital_pendiente,a, cuota_capital, cuota_interes);
                self.tabla_amort_inicial.push(cuota);
                capital_pendiente = redondea_dos_decimales(capital_pendiente - cuota_capital);
                fecha_cuota = lib::add_month(fecha_cuota);
                meses_restantes_antes -= 1;
            }
        }
        fn calcula_tabla_amort_con_actualizaciones_euribor(&mut self) {
            self.calcula_amort_primer_periodo();
            let ult_cuota = self.tabla_amort_con_actualizaciones_euribor.cuotas.last_mut().unwrap();
            let interes = ult_cuota.i;
            let fecha_prox_vencim = lib::add_month(ult_cuota.fecha);
            if fecha_prox_vencim < Utc::today() {

            }
        }
}


mod tests {
    use super::*;

    #[test]
    fn test_hipoteca() {
        let nombre = String::from("Prueba");
        let fecha = Utc.ymd(2004,3,17);
        let mut h1= Hipoteca::new(nombre, fecha, 84140.0, 0.04,
            300,6,12,0.01, 
            0.04, 0.12);
    }
    #[test]
    fn test_calcula_tabla_amort_inicial() {
        let nombre = String::from("Prueba");
        let fecha = Utc.ymd(2004,3,17);
        let mut h1= Hipoteca::new(nombre, fecha, 84140.0, 0.04,
            300,6,12,0.01, 
            0.04, 0.12);
        h1.calcula_tabla_amort_inicial();
        assert_eq!(300, h1.tabla_amort_inicial.len());
        let cuota: &Cuota = &h1.tabla_amort_inicial.cuotas[0];
        assert_eq!(Utc.ymd(2004, 4, 17), cuota.fecha);
        assert_eq!(0.04, cuota.i);
        assert_eq!(300, cuota.meses_restantes_antes);
        assert_eq!(444.12, cuota.cuota_total);
        assert_eq!(163.65, cuota.cuota_capital);
        assert_eq!(280.47, cuota.cuota_interes);
        let mes = usize::try_from(h1.meses_hasta_primera_revision).ok().unwrap();
        let cuota: &Cuota = &h1.tabla_amort_inicial.cuotas[mes-1];
        assert_eq!(Utc.ymd(2004, 9, 17), cuota.fecha);
        assert_eq!(0.04, cuota.i);
        assert_eq!(295, cuota.meses_restantes_antes);
        assert_eq!(444.12, cuota.cuota_total);
        assert_eq!(166.40, cuota.cuota_capital);
        assert_eq!(277.72, cuota.cuota_interes);
        let mes = usize::try_from(h1.meses).ok().unwrap();
        let cuota: &Cuota = &h1.tabla_amort_inicial.cuotas[mes-1];
        assert_eq!(Utc.ymd(2029, 3, 17), cuota.fecha);
        assert_eq!(0.04, cuota.i);
        assert_eq!(1, cuota.meses_restantes_antes);
        assert_eq!(445.12, cuota.cuota_total);
        assert_eq!(443.64, cuota.cuota_capital);
        assert_eq!(1.48, cuota.cuota_interes);
    }
    #[test]
    fn test_calcula_amort_primer_periodo() {
        let nombre = String::from("Prueba");
        let fecha = Utc.ymd(2004,3,17);
        let mut h1= Hipoteca::new(nombre, fecha, 84140.0, 0.04,
            300,6,12,0.01, 
            0.04, 0.12);
        h1.calcula_amort_primer_periodo();
        assert_eq!(6, h1.tabla_amort_inicial.len());
        let cuota: &Cuota = &h1.tabla_amort_inicial.cuotas[0];
        assert_eq!(Utc.ymd(2004, 4, 17), cuota.fecha);
        assert_eq!(0.04, cuota.i);
        assert_eq!(300, cuota.meses_restantes_antes);
        assert_eq!(444.12, cuota.cuota_total);
        assert_eq!(163.65, cuota.cuota_capital);
        assert_eq!(280.47, cuota.cuota_interes);
        let mes = usize::try_from(h1.meses_hasta_primera_revision).ok().unwrap();
        let cuota: &Cuota = &h1.tabla_amort_inicial.cuotas[mes-1];
        assert_eq!(Utc.ymd(2004, 9, 17), cuota.fecha);
        assert_eq!(0.04, cuota.i);
        assert_eq!(295, cuota.meses_restantes_antes);
        assert_eq!(444.12, cuota.cuota_total);
        assert_eq!(166.40, cuota.cuota_capital);
        assert_eq!(277.72, cuota.cuota_interes);

    }

}
