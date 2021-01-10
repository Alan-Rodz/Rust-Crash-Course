//Las variables guardan informacion primitiva o referencias a otra informacion
//Las variables son inmutables por default (por default no podemos reasignarlas)
//Rust es un lenguaje de programacion block-scoped

pub fn run()
{
    let nombre = "Juan";

    //Si agregamos el prefijo mut, la variable se vuelve mutable
    let mut edad = 37;
    println!("Mi nombre es {} y tengo {} años", nombre, edad);
    edad = 38;
    println!("Mi nombre es {} y tengo {} años", nombre, edad);

    //Definimos una variable constante (indicamos que es un entero de 32 bits)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Podemos declarar varias variables al mismo tiempo
    let ( mi_nombre, mi_edad)  = ("Juan", 37);
    println!("{} tiene  {} años de edad", mi_nombre, mi_edad);
}