use chrono::prelude::*;
use super::lib::*;

// Las cuotas son cada uno de los pagos mensuales de la hipoteca
pub struct Cuota {
    pub fecha: Date<Utc>, // Fecha en la que se debe abonar la cuota
    pub cap_pendiente: f64, // Capital pendiente de amortizar antes de pagar esta cuota
    pub i: f64, // Tipo de interes nominal anual vigente en ese momento
    pub meses: i32,
}

impl Cuota {
    pub fn new(fecha: Date<Utc>, cap_pendiente:f64, i:f64, meses: i32) -> Self {
        Cuota { fecha, cap_pendiente, i, meses}
    }
    pub fn cuota_total(&self) -> f64 {
        let i_mensual: f64 = self.i/12.0;
        let a: f64 = self.cap_pendiente * i_mensual / (1.0 - (1.0+i_mensual).powi(-self.meses));
        redondea_dos_decimales(a)
    }
    pub fn cuota_intereses(&self) -> f64 {
        let i_mensual: f64 = self.i/12.0;
        let a: f64 = self.cap_pendiente * i_mensual;
        redondea_dos_decimales(a)
    }
    pub fn cuota_capital(&self) -> f64 {
        redondea_dos_decimales(self.cuota_total() - self.cuota_intereses())
    }
    pub fn cap_pendiente_after(&self) -> f64 {
        redondea_dos_decimales(self.cap_pendiente - self.cuota_capital())
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let cuota = Cuota::new(Utc.ymd(2022, 7, 17), 
            100000.0, 0.04, 25);
        assert_eq!(2022, cuota.fecha.year());
        assert_eq!(7, cuota.fecha.month());
        assert_eq!(17, cuota.fecha.day());
        assert_eq!(100000.0, cuota.cap_pendiente);
        assert_eq!(0.04, cuota.i);
        assert_eq!(25, cuota.meses);
        
    }
    #[test]
    fn test_cuota_total() {
        let cuota = Cuota::new(Utc.ymd(2004, 4, 17), 84140.0, 0.04, 300);
        assert_eq!(444.12, cuota.cuota_total());
        let cuota = Cuota::new(Utc.ymd(2004, 4, 17), 443.64, 0.04, 1);
        assert_eq!(445.12, cuota.cuota_total());
    }
    #[test]
    fn test_cuota_intereses() {
        let cuota = Cuota::new(Utc.ymd(2004, 4, 17), 84140.0, 0.04, 300);
        assert_eq!(280.47, cuota.cuota_intereses());
        let cuota = Cuota::new(Utc.ymd(2004, 4, 17), 443.64, 0.04, 1);
        assert_eq!(1.48, cuota.cuota_intereses());   
    }
    #[test]
    fn test_cuota_capital() {
        let cuota = Cuota::new(Utc.ymd(2004, 4, 17), 84140.0, 0.04, 300);
        assert_eq!(163.65, cuota.cuota_capital());
        let cuota = Cuota::new(Utc.ymd(2004, 4, 17), 443.64, 0.04, 1);
        assert_eq!(443.64, cuota.cuota_capital());
    }
    #[test]
    fn test_cap_pendiente_after() {
        let cuota = Cuota::new(Utc.ymd(2004, 4, 17), 84140.0, 0.04, 300);
        assert_eq!(83976.35, cuota.cap_pendiente_after());
        let cuota = Cuota::new(Utc.ymd(2004, 4, 17), 443.64, 0.04, 1);
        assert_eq!(0.0, cuota.cap_pendiente_after());
    }

}