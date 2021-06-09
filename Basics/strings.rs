//String primitivo = inmutable, de tamaño fijo
//String normal = mutable, estructura de datos en el heap, usado cuando tenemos quem modificar la informacion del string

pub fn run()
{
    //Primitivo
    let hola = "Hola!";

    //Mutable
    let mut hola2 = String::from("Hola!!");
 
    println!("{} {}", hola, hola2);

    //Obteniendo longitud de strings
    println!("Longitudes: {}, {}", hola.len(), hola2.len());

    //Agregando un char a un string
    hola2.push('!');

    //Agregando varios elementos a un string
    hola2.push_str("!!!!!");

    println!("{} {}", hola, hola2);

    //Capacidad en bytes
    println!("Capacidad: {}", hola2.capacity());

    //Checando si un string esta vacio
    println!("Esta vacio: {}", hola2.is_empty());

    //Checando si un string contiene un substring
    println!("Contiene '!!!': {}", hola2.contains("!!!"));

    //Reemplazando substrings
    println!("Reemplazar: {} ", hola2.replace("Hola", "Adios"));

    let lorem = String::from("lorem ipsum lorem ipsum");
    //Iterando a través del string
    for palabra in lorem.split_whitespace()
    {
        println!("{}", palabra);
    }

    //Creando string con capacidad determinada
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    //assert_eq!(3, s.len());

}
