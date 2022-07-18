use chrono::prelude::*;
use chrono::Utc;

use super::lib::*;
use super::cuota::Cuota;
use super::tabla_amort::TablaAmortizacion;
use super::euribor_data::EuriborData;

pub struct Hipoteca {
    /// Usado para los nombres de los ficheros que se creen
    pub nombre_operacion: String, 
    /// Fecha inicial de la hipoteca establecida en la escritura
    pub fecha_escritura: Date<Utc>, 
    /// Capital inicial prestado, según establece la escritura
    pub capital_prestado: f64, 
    /// Tipo de interés nominal anual establecido por la escritura
    pub tipo_interes_anual: f64, 
    /// Meses para amortizar la hipoteca
    pub meses: i32, 
    /// Fecha último vencimiento
    pub fecha_ultimo_vencimiento: Date<Utc>,
    /// Meses de aplicación del tipo de interes nominal inicial, antes 
    /// de la primera actualización
    pub meses_hasta_primera_revision: i32, 
    /// Meses entre actualizaciones de tipos interés
    pub intervalo_revisiones: i32,
    /// Incremento aplicado sobre el euribor en las revisiones 
    pub incremento_euribor: f64, 
    /// Tipo mínimo establecido en las cláusulas de la hipoteca
    pub i_min: f64, 
    /// Tipo máximo establecido en las cláusulas de la hipoteca
    pub i_max: f64, 
    /// Tabla de amortización completa, pero con todas
    /// las cuotas calculadas con el interés inicial i
    pub tabla_amort_sin_actualizacion: TablaAmortizacion,
    /// Tabla de amortización con las actualizaciones del Euribor
    pub tabla_amort_con_actualizacion_euribor: TablaAmortizacion, 
}
impl Hipoteca {
        pub fn new(nombre_operacion: String, fecha_escritura: Date<Utc>, capital_prestado: f64, 
            tipo_interes_anual: f64, meses: i32, meses_hasta_primera_revision: i32, 
            intervalo_revisiones: i32, incremento_euribor: f64, 
            i_min: f64, i_max: f64) -> Self {
            Hipoteca { 
                nombre_operacion,
                fecha_escritura, 
                capital_prestado, 
                tipo_interes_anual,  
                meses, 
                fecha_ultimo_vencimiento: add_n_months(fecha_escritura, meses),
                meses_hasta_primera_revision, 
                intervalo_revisiones,  
                incremento_euribor, 
                i_min, 
                i_max, 
                tabla_amort_sin_actualizacion: TablaAmortizacion::new(),
                tabla_amort_con_actualizacion_euribor: TablaAmortizacion::new(),
            }
        }
        /// Calcula la tabla de amortización con los datos iniciales de 
        /// la hipoteca, sin ningún tipo de actualizaciones del tipo 
        /// de interés
        pub fn calcula_tabla_amort_sin_actualizacion(&mut self) -> TablaAmortizacion {
            let mut tabla : TablaAmortizacion = TablaAmortizacion::new();
            let mut fecha: Date<Utc> = add_one_month(self.fecha_escritura);            
            let cuota_total = importe_cuota_mensual(self.capital_prestado, self.tipo_interes_anual, self.meses);
            let tipo_interes = self.tipo_interes_anual;
            let mut meses_restantes_antes = self.meses;
            let mut capital_pendiente_antes: f64 = self.capital_prestado;
            for _i in 0..self.meses {
                let cuota_interes = intereses_mes(capital_pendiente_antes, tipo_interes);
                let cuota_capital = redondea_dos_decimales(cuota_total-cuota_interes);
                let cuota: Cuota = Cuota::new(fecha, self.tipo_interes_anual, meses_restantes_antes,
                    capital_pendiente_antes,cuota_total, cuota_capital, cuota_interes);
                tabla.push(cuota);

                capital_pendiente_antes = redondea_dos_decimales(capital_pendiente_antes - cuota_capital);
                fecha = add_one_month(fecha);
                meses_restantes_antes -= 1;
            }
            // Ajuste de la ultima cuota por descuadres de redondeo
            if capital_pendiente_antes != 0.0 {
                let ult_cuota = tabla.cuotas.last_mut().unwrap();
                //ult_cuota.cap_pendiente_antes += capital_pendiente_antes;
                ult_cuota.cuota_total += capital_pendiente_antes;
                ult_cuota.cuota_capital += capital_pendiente_antes;
            } 
            tabla
        }

        pub fn calcula_tabla_amort_con_actualizacion_euribor(&mut self) -> TablaAmortizacion {
            let ed = EuriborData::new();
            let mut tabla = self.calcula_amort_primer_periodo();
            let ult_cuota = tabla.cuotas.last_mut().unwrap();
            let mut fecha_prox_vencim = add_one_month(ult_cuota.fecha);
            let mut meses_restantes_antes = ult_cuota.meses_restantes_antes-1;
            let mut cap_pendiente_antes = ult_cuota.cap_pendiente_despues();
            // Ir calculando periodos con actualización euribor en cada periodo
            while fecha_prox_vencim <= self.fecha_ultimo_vencimiento {
                // TODO Actualizar euribor + increm, imax, imin
                let tipo_interes = ed.actualiza_euribor(fecha_prox_vencim, 
                    self.incremento_euribor, self.i_min, self.i_max);
                let cuota_total: f64 = importe_cuota_mensual(cap_pendiente_antes, tipo_interes, meses_restantes_antes); 
                for _i in 0..self.intervalo_revisiones {
                    let cuota_intereses = intereses_mes(cap_pendiente_antes, tipo_interes);
                    let cuota_capital = cuota_total - cuota_intereses; 
                    let cuota = Cuota::new(fecha_prox_vencim, tipo_interes, 
                        meses_restantes_antes,cap_pendiente_antes,cuota_total, cuota_capital,
                        cuota_intereses);
                    tabla.push(cuota);
                    cap_pendiente_antes = redondea_dos_decimales(cap_pendiente_antes - cuota_capital);
                    meses_restantes_antes -= 1;
                    fecha_prox_vencim = add_one_month(fecha_prox_vencim);
                    if fecha_prox_vencim > self.fecha_ultimo_vencimiento {
                        break;
                    }
                }
            }
            // Ajuste de la ultima cuota por descuadres de redondeo
            if cap_pendiente_antes != 0.0 {
                let ult_cuota = tabla.cuotas.last_mut().unwrap();
                //ult_cuota.cap_pendiente_antes += cap_pendiente_antes;
                ult_cuota.cuota_total += cap_pendiente_antes;
                ult_cuota.cuota_capital += cap_pendiente_antes;
            }
            tabla
        }
        /// Calcula una tabla de amortización para los meses iniciales, 
        /// los que marca la escritura antes de la primera actualización
        /// del euribor
        fn calcula_amort_primer_periodo(&mut self) -> TablaAmortizacion {
            let mut tabla = TablaAmortizacion::new();
            let cuota_total = importe_cuota_mensual(self.capital_prestado, self.tipo_interes_anual, self.meses);
            let mut capital_pendiente: f64 = self.capital_prestado;
            let mut fecha_cuota: Date<Utc> = add_one_month(self.fecha_escritura);            
            let mut meses_restantes_antes = self.meses;
            for _i in 0..self.meses_hasta_primera_revision {
                if fecha_cuota > self.fecha_ultimo_vencimiento {
                    return tabla
                }
                let cuota_interes = redondea_dos_decimales(capital_pendiente*self.tipo_interes_anual/12.0);
                let cuota_capital = redondea_dos_decimales(cuota_total-cuota_interes);
                let cuota: Cuota = Cuota::new(fecha_cuota, self.tipo_interes_anual, meses_restantes_antes,
                    capital_pendiente,cuota_total, cuota_capital, cuota_interes);
                tabla.push(cuota);
                capital_pendiente = redondea_dos_decimales(capital_pendiente - cuota_capital);
                fecha_cuota = add_one_month(fecha_cuota);
                meses_restantes_antes -= 1;
            }
            tabla
        }

}


mod tests {
    use super::*;

    #[test]
    fn test_hipoteca() {
        let nombre = String::from("Prueba");
        let fecha = Utc.ymd(2004,3,17);
        let h1= Hipoteca::new(nombre, fecha, 84140.0, 0.04,
            300,6,12,0.01, 
            0.04, 0.12);
    }
    #[test]
    fn test_importe_cuota_mensual() {
        let x = importe_cuota_mensual(84140.0, 0.04, 300);
        assert_eq!(444.12, x);
    }
    #[test]
    fn test_calcula_tabla_amort_sin_actualizacion() {
        let nombre = String::from("Prueba");
        let fecha = Utc.ymd(2004,3,17);
        let mut h1= Hipoteca::new(nombre, fecha, 84140.0, 0.04,
            300,6,12,0.01, 
            0.04, 0.12);
        h1.tabla_amort_sin_actualizacion = h1.calcula_tabla_amort_sin_actualizacion();
        assert_eq!(300, h1.tabla_amort_sin_actualizacion.len());
        let cuota: &Cuota = &h1.tabla_amort_sin_actualizacion.cuotas[0];
        assert_eq!(Utc.ymd(2004, 4, 17), cuota.fecha);
        assert_eq!(0.04, cuota.i);
        assert_eq!(300, cuota.meses_restantes_antes);
        assert_eq!(444.12, cuota.cuota_total);
        assert_eq!(163.65, cuota.cuota_capital);
        assert_eq!(280.47, cuota.cuota_interes);
        let mes = usize::try_from(h1.meses_hasta_primera_revision).ok().unwrap();
        let cuota: &Cuota = &h1.tabla_amort_sin_actualizacion.cuotas[mes-1];
        assert_eq!(Utc.ymd(2004, 9, 17), cuota.fecha);
        assert_eq!(0.04, cuota.i);
        assert_eq!(295, cuota.meses_restantes_antes);
        assert_eq!(444.12, cuota.cuota_total);
        assert_eq!(166.40, cuota.cuota_capital);
        assert_eq!(277.72, cuota.cuota_interes);
        let mes = usize::try_from(h1.meses).ok().unwrap();
        let cuota: &Cuota = &h1.tabla_amort_sin_actualizacion.cuotas[mes-1];
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
        h1.tabla_amort_sin_actualizacion = h1.calcula_amort_primer_periodo();
        assert_eq!(6, h1.tabla_amort_sin_actualizacion.len());
        let cuota: &Cuota = &h1.tabla_amort_sin_actualizacion.cuotas[0];
        assert_eq!(Utc.ymd(2004, 4, 17), cuota.fecha);
        assert_eq!(0.04, cuota.i);
        assert_eq!(300, cuota.meses_restantes_antes);
        assert_eq!(444.12, cuota.cuota_total);
        assert_eq!(163.65, cuota.cuota_capital);
        assert_eq!(280.47, cuota.cuota_interes);
        let mes = usize::try_from(h1.meses_hasta_primera_revision).ok().unwrap();
        let cuota: &Cuota = &h1.tabla_amort_sin_actualizacion.cuotas[mes-1];
        assert_eq!(Utc.ymd(2004, 9, 17), cuota.fecha);
        assert_eq!(0.04, cuota.i);
        assert_eq!(295, cuota.meses_restantes_antes);
        assert_eq!(444.12, cuota.cuota_total);
        assert_eq!(166.40, cuota.cuota_capital);
        assert_eq!(277.72, cuota.cuota_interes);

    }

    #[test]
    fn test_calcula_tabla_amort_con_actualizacion_euribor() {
        let nombre = String::from("Prueba");
        let fecha = Utc.ymd(2004,3,17);
        let mut h1= Hipoteca::new(nombre, fecha, 84140.0, 0.04,
            300,6,12,0.01, 
            0.04, 0.12);        
        h1.tabla_amort_con_actualizacion_euribor = h1.calcula_tabla_amort_con_actualizacion_euribor();
        h1.tabla_amort_con_actualizacion_euribor.disp();
    }
}
