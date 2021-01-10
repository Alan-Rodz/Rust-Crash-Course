//Así es como importamos otros archivos
mod print;
mod variables;
mod tipos;
mod strings;
mod tuplas;
mod arreglos;
mod vectores;
mod condicionales;
mod loops;
mod funciones;
mod punteros_referencias;
mod structs;
mod enums;
mod cli;

fn main() 
{
    //Podemos acceder a funciones de otros archivos así
    print::run();
    variables::run();
    tipos::run();
    strings::run();
    tuplas::run();
    arreglos::run();
    vectores::run();
    condicionales::run();
    loops::run();
    funciones::run();
    punteros_referencias::run();
    structs::run();
    enums::run();
    cli::run();
}
