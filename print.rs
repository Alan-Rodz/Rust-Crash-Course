//Pub significa public
pub fn run()
{
    //Imprimiendo strings
    println!("Hola desde print.rs!");

    //Imprimiendo numeros
    println!("{} dias", 31);

    //Imprimiendo con varios placeholders
    println!("{} es de {}", "Juan", "Guadalajara");

    //Argumentos posicionales
    println!("{0} es de {1} y a {0} le gusta {2}", "Juan", "Guadalajara", "Nadar");

    //Argumentos nombrados
    println!("A {nombre} le gusta {actividad}", nombre="Juan", actividad="Cocinar");

    //Placeholder traits
    println!("Binario:{:b}, Hexadecimal:{:x}, Octal:{:o}", 10, 10, 10);

    //Debug traits
    println!("{:?}", (12, true, "hola"));

    //Matemática básica
    println!("10+10 = {}", 10+10);
}