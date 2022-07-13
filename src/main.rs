mod libs;

use chrono::prelude::*;
use chrono::Utc;
use libs::hipoteca::*;

fn main() {
    let mut h1= Hipoteca::new(Utc.ymd(2004,3,17), 84140.0, 0.04,300,6,12,0.01);
    h1.calcula_cuotas();
    h1.disp_cuadro_amortizacion();
}
