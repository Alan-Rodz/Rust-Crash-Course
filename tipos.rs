//Tipos de variables que podemos usar
/*
    Tipos primitivos:
    Unsigned significa que no pueden tomar valores negativos
    Enteros: u8 (unsigned-8bit), i8, u16, i16, u32, i32, u64, i64, u128, i128 (numero de bits que toman en memoria)
    Floats: f32, f64
    Boolean: (bool)
    Caracteres: (char)
    Tuplas
    Arreglos
*/

/*
    Rust es un lenguaje tipado estaticamente, por lo que debe saber los tipos de todas las variables a la hora de
    la compilación, pero aún así el compilador puede inferir qué tipo queremos usar basado en el valor de la variable
    y cómo la usamos.
*/

pub fn run()
{
    //Por default es i32
    let x = 1;

    //Por default es f64
    let y = 2.5;

    //Tipado explicito
    let z:i64 = 123456789987654321;
    
    //Encontrar el tamaño maximo de un tipo
    println!("Tamaño maximo i32: {}", std::i32::MAX);
    println!("Tamaño maximo i64: {}", std::i64::MAX);

    //Booleanos
    let esta_activo: bool = true;
    println!("{:?}", (x,y,z, esta_activo));

    //Obteniendo booleanos a partir de expresiones
    let es_mayor_que = 10>5;
    println!("{:?}", es_mayor_que);

    //Caracteres
    let a1 = 'a';
    let cara = '\u{1F600}';
    println!("{:?}", a1);
    println!("{}", cara);

}