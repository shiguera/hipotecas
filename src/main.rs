mod libs;

use chrono::prelude::*;
use chrono::Utc;
use libs::hipoteca::*;

fn main() {
    let nombre = String::from("Prueba");
    let mut h1= Hipoteca::new(nombre, Utc.ymd(2004,3,17), 84140.0, 0.04,300,6,12,0.01);
    h1.calcula_cuotas();
    h1.disp_cuadro_amortizacion();
}
