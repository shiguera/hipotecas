use chrono::prelude::*;


pub struct Novacion {
    fecha_novacion: Date<Utc>,
    incremento_capital: f64,

}

impl Novacion {
    fn new(fecha_novacion: Date<Utc>, incremento_capital: f64) -> Self {
        Novacion {fecha_novacion, incremento_capital}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fecha = Utc.ymd(2006, 6, 22);
        let incremento = 32498.24;
        let nova1 = Novacion::new(fecha, incremento);
        assert_eq!(Utc.ymd(2006, 6, 22), nova1.fecha_novacion);
        assert_eq!(32498.24, nova1.incremento_capital);
    }
}