//Los enums funcionan como en otros lenguajes de programacion

enum Movimiento 
{
    //Variantes del enum
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

fn mover(m: Movimiento) 
{
    //Realizamos una accion dependiendo del movimiento
    //Match es como un switch
    match m 
    {
        Movimiento::Arriba => println!("El personaje se mueve hacia Arriba!"),
        Movimiento::Abajo => println!("El personaje se mueve hacia Abajo!"),
        Movimiento::Izquierda => println!("El personaje se mueve hacia la Izquierda!"),
        Movimiento::Derecha => println!("El personaje se mueve hacia Derecha!"),
    }
}

pub fn run() 
{
    let personaje1 = Movimiento::Izquierda;
    let personaje2 = Movimiento::Arriba;
    let personaje3 = Movimiento::Derecha;
    let personaje4 = Movimiento::Abajo;

    mover(personaje1);
    mover(personaje2);
    mover(personaje3);
    mover(personaje4);
}
