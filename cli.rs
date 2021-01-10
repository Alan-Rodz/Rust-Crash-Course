//Como interactuar con la consola a la hora de ejecutar nuestros programas de Rust
use std::env;

pub fn run()
{
    //Env nos permite recolectar los argumentos que pasamos en la consola
    //El primer elemento siempre es la dirección del ejecutable 
    //Si hacemos algo como cargo run hola entonces estaremos añadiendo "hola" al vector
    let args: Vec<String> = env::args().collect();
    let comando = args[1].clone();
    println!("Argumentos: {:?}", args);
    println!("Comando provisto a través de la consola: {:?}", comando);

    let nombre = "Miguel";
    if comando == "hola" 
    {
        println!("Hola {}!", nombre);
    }
}