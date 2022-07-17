mod libs;

use chrono::prelude::*;
use chrono::Utc;
use libs::hipoteca::*;
use std::io;
use colored::*;

fn main() {
    let nombre = String::from("Prueba");
    let mut h1= Hipoteca::new(nombre, Utc.ymd(2004,
        3,17), 84140.0, 0.04,300,6,
        12,0.01, 0.04, 0.12);

    let mut nombre = String::new();
    println!("{}", "Nombre: ".yellow());
    let result = io::stdin().read_line(&mut nombre);
    println!("{:?}",result);
    println!("{:?}", nombre); // Incluye el caracter \n
    println!("{:?}",nombre.trim()); // trim() quita el \n
}
