use chrono::prelude::*;
use super::lib::*;

// Las cuotas son cada uno de los pagos mensuales de la hipoteca
pub struct Cuota {
    /// Los datos se calculan al calcular la tabla de amortización
    /// Si se calcula cada cuota con los meses que le quedan y el 
    /// tipo de interés que le corresponde, sale un pequeño descuadre
    /// por el redondeo de los decimales
    ///
    /// Fecha en la que se debe abonar la cuota
    pub fecha: Date<Utc>,
    /// Tipo de interes nominal anual vigente en ese momento 
    pub i: f64, 
    /// Meses restantes, incluyendo este
    pub meses_restantes_antes: i32,
    /// Capital pendiente de amortizar antes de pagar esta cuota 
    pub cap_pendiente_antes: f64,
    /// Importe de la cuota a pagar 
    pub cuota_total: f64, 
    /// Parte de capital de la cuota
    pub cuota_capital: f64, 
    /// Parte de intereses de la cuota
    pub cuota_interes: f64, 
}

impl Cuota {
    pub fn new(fecha: Date<Utc>,i:f64, meses_restantes_antes: i32,
        cap_pendiente_antes:f64, cuota_total: f64, cuota_capital: f64, 
        cuota_interes: f64) -> Self {
        Cuota { fecha, i, meses_restantes_antes, cap_pendiente_antes,
             cuota_total, cuota_capital, cuota_interes}
    }
    /// Calcula el capital pendient de amortización
    /// despues de pagar esta cuota
    pub fn cap_pendiente_despues(&self) -> f64 {
        redondea_dos_decimales(self.cap_pendiente_antes - 
            self.cuota_capital)
    }    
    /// Muestra en pantalla la instancia de Cuota
    pub fn disp(&self) {
        println!("{}", self.to_string());
    }
    pub fn to_string(&self) -> String {
        format!("{}/{}/{} {:.5} {} {:.2} {:.2} {:.2} {:.2} {:.2}", self.fecha.day(), self.fecha.month(),
            self.fecha.year(), self.i, self.meses_restantes_antes,
            self.cap_pendiente_antes, self.cuota_total, self.cuota_capital, self.cuota_interes, 
            self.cap_pendiente_despues())
    }
    pub fn to_csv_string(&self) -> String {
        let i = self.i.to_string().replace(".", ",");
        let meses = self.meses_restantes_antes.to_string();
        let cap_pdte_antes = self.cap_pendiente_antes.to_string().replace(".", ",");
        let cuota_total = self.cuota_total.to_string().replace(".", ",");
        let cuota_capital = self.cuota_capital.to_string().replace(".", ",");
        let cuota_interes = self.cuota_interes.to_string().replace(".", ",");
        let cap_pdte_despues = self.cap_pendiente_despues().to_string().replace(".", ",");
        format!("{}/{}/{}; {}; {}; {}; {}; {}; {}; {}", 
            self.fecha.day(), self.fecha.month(), self.fecha.year(), 
            i, meses, cap_pdte_antes, cuota_total, cuota_capital, 
            cuota_interes, cap_pdte_despues)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let cuota = Cuota::new(
            Utc.ymd(2004, 4, 17),0.04,
            300, 84140.0, 444.12, 163.65, 280.47); 
        assert_eq!(2004, cuota.fecha.year());
        assert_eq!(4, cuota.fecha.month());
        assert_eq!(17, cuota.fecha.day());
        assert_eq!(84140.0, cuota.cap_pendiente_antes);
        assert_eq!(0.04, cuota.i);
        assert_eq!(300, cuota.meses_restantes_antes);
        
    }
    #[test]
    fn test_cuota_pendiente_despues() {
        let cuota = Cuota::new(
            Utc.ymd(2004, 4, 17),0.04,
            300, 84140.0, 444.12, 163.65, 280.47); 
        assert_eq!(83976.35, cuota.cap_pendiente_despues());
    }
    #[test]
    fn test_disp() {
        let cuota = Cuota::new(
            Utc.ymd(2004, 4, 17),0.04,
            300, 84140.0, 444.12, 163.65, 280.47); 
        cuota.disp();
    }
    #[test]
    fn test_to_csv_string() {
        let x = 84140.0;
        let y = x.to_string();
        println!("{}", y);
        let z = y.replace(".", ",");
        println!("{}", z);
    }
}