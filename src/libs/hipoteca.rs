use chrono::prelude::*;
use chrono::Utc;

use super::lib::*;
use super::cuota::Cuota;
use super::tabla_amort::TablaAmortizacion;
use super::euribor_data::EuriborData;
use super::novacion::Novacion;

pub struct Hipoteca {
    /// Usado para los nombres de los ficheros que se creen
    pub nombre_operacion: String, 
    /// Fecha inicial de la hipoteca establecida en la escritura
    pub fecha_escritura: Date<Utc>, 
    /// Fecha de pago de la primera cuota
    pub fecha_primera_cuota: Date<Utc>,
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
    /// Fechas en las que se deben producir las revisiones
    pub fechas_revisiones: Vec<Date<Utc>>,
    /// Tipo mínimo establecido en las cláusulas de la hipoteca
    pub i_min: f64, 
    /// Tipo máximo establecido en las cláusulas de la hipoteca
    pub i_max: f64, 
    /// Fecha en la que se produjo el impago
    pub fecha_impago: Option<Date<Utc>>,
    /// Fecha en la que se produjo la resolución del contrato
    pub fecha_resolucion: Option<Date<Utc>>,
    /// Novaciones: lista de novaciones y ampliaciones
    pub novaciones: Vec<Novacion>,
    /// Tabla de amortización completa, pero con todas
    /// las cuotas calculadas con el interés inicial i
    pub tabla_amort_sin_actualizacion: TablaAmortizacion,
    /// Tabla de amortización con las actualizaciones del Euribor
    pub tabla_amort_con_actualizacion_euribor: TablaAmortizacion, 
    /// Tabla de amortización para el periodo del impago a la resolución
    pub tabla_amort_impago: TablaAmortizacion,
}
impl Hipoteca {
        
    /// Crea una instancia de Hipoteca
    pub fn new(nombre_operacion: String, fecha_escritura: Date<Utc>, 
            fecha_primera_cuota: Date<Utc>, capital_prestado: f64, 
            tipo_interes_anual: f64, meses: i32, meses_hasta_primera_revision: i32, 
            intervalo_revisiones: i32, incremento_euribor: f64, 
            i_min: f64, i_max: f64, fecha_impago:Option<Date<Utc>>, fecha_resolucion:Option<Date<Utc>>) -> Self {
        let mut h = Hipoteca { 
            nombre_operacion,
            fecha_escritura, 
            fecha_primera_cuota,
            capital_prestado, 
            tipo_interes_anual,  
            meses, 
            fecha_ultimo_vencimiento: add_n_months(fecha_escritura, meses),
            meses_hasta_primera_revision, 
            intervalo_revisiones,  
            incremento_euribor, 
            fechas_revisiones: Vec::<Date<Utc>>::new(), 
            i_min, 
            i_max, 
            fecha_impago,
            fecha_resolucion,
            novaciones: Vec::<Novacion>::new(),
            tabla_amort_sin_actualizacion: TablaAmortizacion::new(),
            tabla_amort_con_actualizacion_euribor: TablaAmortizacion::new(),
            tabla_amort_impago: TablaAmortizacion::new(),
        };
        h.tabla_amort_sin_actualizacion = h.calcula_tabla_amort_sin_actualizacion();
        h.tabla_amort_con_actualizacion_euribor = h.calcula_tabla_amort_con_actualizacion_euribor();
        h.calcula_fechas_revisiones();
        h        
    }

    /// Calcula la tabla de amortización con los datos iniciales de 
    /// la hipoteca, sin ningún tipo de actualizaciones del tipo 
    /// de interés
    pub fn calcula_tabla_amort_sin_actualizacion(&mut self) -> TablaAmortizacion {
        calcula_tabla_amortizacion(self.capital_prestado, self.tipo_interes_anual,
            self.meses, self.fecha_primera_cuota)        
    }
    fn calcula_fechas_revisiones(&mut self)  {
        self.fechas_revisiones = Vec::<Date<Utc>>::new();
        let fecha_primera_revision = 
            add_n_months(self.fecha_primera_cuota, self.meses_hasta_primera_revision);
        self.fechas_revisiones.push(fecha_primera_revision);
        let mut fecha_revision = 
            add_n_months(fecha_primera_revision, self.intervalo_revisiones);
        while fecha_revision <= self.fecha_ultimo_vencimiento {
            self.fechas_revisiones.push(fecha_revision.clone());
            fecha_revision = add_n_months(fecha_revision, self.intervalo_revisiones);
        }
    }
    /// Calcula la tabla de amortización actualizando
    /// con el euribor en cada periodo
    pub fn calcula_tabla_amort_con_actualizacion_euribor(&mut self) -> TablaAmortizacion {
        let mut tabla = self.calcula_tabla_amort_sin_actualizacion();
        self.calcula_fechas_revisiones();
        for i in &self.fechas_revisiones {
            tabla = tabla.actualiza_euribor(
                i.clone(), self.incremento_euribor, self.i_min, self.i_max);
        }
        tabla
    }

    

    /// Calcula novaciones
    pub fn calcula_novaciones(&mut self) {
        if self.novaciones.len() > 0 {
            for i in 0..self.novaciones.len() {
                //let &novacion = self.novaciones[i];
                //self.calcula_novacion(novacion);
            }
        }
    }
    fn calcula_novacion(&mut self, novacion: Novacion) {
        // Tabla de cuotas hasta la fecha de la novación
        let mut tabla_antes: TablaAmortizacion = TablaAmortizacion::new();
        self.tabla_amort_con_actualizacion_euribor.cuotas
            .iter()
            .filter(|x| x.fecha <= novacion.fecha_novacion)
            .for_each(|x| tabla_antes.push(x.clone()));
        if tabla_antes.len() == 0 {
            // Si no hay cuotas anteriores a la novación volver
            return
        }
        // Tabla de las cuotas posteriores a la novación
        let mut tabla_despues = TablaAmortizacion::new();
        self.tabla_amort_con_actualizacion_euribor.cuotas
            .iter()
            .filter(|x| x.fecha > novacion.fecha_novacion)
            .for_each(|x| tabla_despues.push(x.clone()));
        if tabla_despues.len() == 0 {
            // Si no hay cuotas posteriores a la novación volver
            return
        }
        // Capital pendiente a partir de la fecha de la novación
        let cap_pendiente = tabla_antes.cuotas.last().unwrap()
            .cap_pendiente_despues() + novacion.incremento_capital;
        //let meses_restantes = primera_cuota_tras_novacion.meses_restantes_antes;
        // Recalcular la hipoteca como una nueva hipoteca en el periodo restante
        //let nueva_h = Hipoteca::new("Nueva", )
        

    }
    /// Calcula la tabla de amortización desde el momento del 
    /// impago hasta la fecha de resolución de la hipoteca
    pub fn calcula_tabla_impago(&mut self) -> TablaAmortizacion  {
        let mut t  =  TablaAmortizacion::new();
        if self.fecha_impago.is_none() {
            return t
        }
        let _cuotas_impago = self.tabla_amort_con_actualizacion_euribor
            .cuotas.iter()
            .filter(|x| x.fecha>=self.fecha_impago.unwrap() && 
                x.fecha<=self.fecha_resolucion.unwrap())
            .for_each(|x| t.cuotas.push(x.clone()));
        //println!("impagos:{}", t.cuotas.len());
        let ultima_cuota: &Cuota = t.cuotas.last().unwrap();
        if ultima_cuota.fecha < self.fecha_resolucion.unwrap() {
            let cap_pdte = ultima_cuota.cap_pendiente_despues();
            let dias = (self.fecha_resolucion.unwrap() - ultima_cuota.fecha).num_days();
            let intereses = redondea_dos_decimales(cap_pdte * dias as f64 * ultima_cuota.i / 365.0);
            let cuota_total = cap_pdte + intereses;
            let cuota = Cuota::new(self.fecha_resolucion.unwrap(), ultima_cuota.i,ultima_cuota.meses_restantes_antes,
                cap_pdte, cuota_total, cap_pdte, intereses);
            t.push(cuota);
        }
        t
    }
}


#[cfg(test)]
mod tests {
    //#[allow(unused_imports)]
    use super::*;
    //use chrono::Utc;
    
    fn create_sample_hipoteca() -> Hipoteca {
        let nombre = String::from("Prueba");
        let fecha = Utc.ymd(2004,3,17);
        let h1= Hipoteca::new(nombre, fecha, 
            Utc.ymd(2004, 4, 17),
             10000.0, 0.04,
            5,1,2,0.00, 
            0.00, 0.12, Some(Utc.ymd(2018, 5, 17)),
            Some(Utc.ymd(2022, 8, 5)));
            h1
    }
    
    #[test]
    fn test_pruebas() {
        let nombre = String::from("Prueba");
        let fecha = Utc.ymd(2004,3,17);
        let _h1= Hipoteca::new(nombre, 
            fecha, 
            Utc.ymd(2004, 4, 17), 
            10000.0, 0.04,
            5,1,
            2,0.00, 
            0.00, 0.12, Some(Utc.ymd(2018, 5, 17)),
            Some(Utc.ymd(2022, 8, 5)));
        //_h1.tabla_amort_sin_actualizacion.disp();
        //_h1.tabla_amort_con_actualizacion_euribor.disp();

    }
    
    #[test]
    fn test_new() {
        let h = create_sample_hipoteca();
        assert_eq!(5, h.tabla_amort_sin_actualizacion.len());
        assert_eq!(5, h.tabla_amort_con_actualizacion_euribor.len());
        assert_eq!(2, h.fechas_revisiones.len());
    }
    #[test]
    fn test_calcula_fechas_revisiones() {
        let mut h = create_sample_hipoteca();
        //h.tabla_amort_sin_actualizacion.disp();
        h.calcula_fechas_revisiones();
        assert_eq!(2, h.fechas_revisiones.len());
        //h.fechas_revisiones.iter().for_each(|x| println!("{}", x.to_string()));
        h.intervalo_revisiones = 1;
        h.calcula_fechas_revisiones();
        //h.fechas_revisiones.iter().for_each(|x| println!("{}", x.to_string()));
        assert_eq!(4, h.fechas_revisiones.len());
    }
    #[test] 
    fn test_calcula_tabla_impago() {
        let nombre = String::from("Prueba");
        let fecha = Utc.ymd(2004,3,17);
        let mut h1= Hipoteca::new(nombre, fecha, 
            Utc.ymd(2004, 4, 17), 84140.0, 0.04,
            300,6,12,0.01, 
            0.04, 0.12, Some(Utc.ymd(2018, 5, 17)),
            Some(Utc.ymd(2022, 8, 5)));
        h1.tabla_amort_impago = h1.calcula_tabla_impago();
        //h1.tabla_amort_impago.disp();
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
        let h1= Hipoteca::new(nombre, fecha,
             Utc.ymd(2004, 4, 17), 84140.0, 
             0.04,
            300,6,12,0.01, 
            0.04, 0.12, Some(Utc.ymd(2018, 5, 17)),
            Some(Utc.ymd(2022, 8, 5)));
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
        assert_eq!(445.1, cuota.cuota_total);
        assert_eq!(443.62, cuota.cuota_capital);
        assert_eq!(1.48, cuota.cuota_interes);
        //h1.tabla_amort_sin_actualizacion.disp();
    }

    #[test]
    fn test_calcula_tabla_amort_con_actualizacion_euribor() {
        let h1= create_sample_hipoteca();      
        //h1.tabla_amort_con_actualizacion_euribor.disp();
        assert_eq!(5, h1.tabla_amort_con_actualizacion_euribor.len());
        assert_eq!(0.0, h1.tabla_amort_con_actualizacion_euribor.cuotas.last().unwrap().cap_pendiente_despues());
    }

    #[test]
    fn test_fecha_ult_vto() {
        let nombre = String::from("Prueba");
        let fecha = Utc.ymd(2004,3,17);
        let h1= Hipoteca::new(nombre, fecha,
            Utc.ymd(2004, 04, 17), 84140.0, 0.04,
            300,6,12,0.01, 
            0.04, 0.12, Some(Utc.ymd(2018, 5, 17)),
            Some(Utc.ymd(2022, 8, 5)));
        assert_eq!(Utc.ymd(2029, 3, 17), h1.fecha_ultimo_vencimiento);
    }
}
