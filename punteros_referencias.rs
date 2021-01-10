//Punteros y Referencias

pub fn run()
{
    //Arreglo primitivo
    let arreglo1 = [1,2,3];
    let arreglo2 = arreglo1;

    println!("Valores de arreglos: {:?}", (arreglo1, arreglo2));

    //Con datos no primitivos, si asignamos otra variable a un pedazo de informacion, la primera variable ya no tendrá
    //ese valor, sino que tendremos que usar una referencia (&) para apuntar al recurso

    //Vectores (no son primitivos)
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("Valores de Vectores: {:?}", (&vec1, vec2));
}