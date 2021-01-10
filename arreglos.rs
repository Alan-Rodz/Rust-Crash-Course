//Los arreglos son listas de tamaño fijo donde los elementos son del mismo tipo     

use std::mem;

pub fn run()
{
    //Tipo de dato y longitud del arreglo
    let mut numeros:[i32; 5] = [1,2,3,4,5];

    //Imprimiendo todo el arreglo
    println!("{:?}", numeros);

    //Obteniendo un valor particular
    println!("{}", numeros[0]);

    //Aunque no podemos añadir, si podemos cambiar los valores del arreglo
    numeros[2] = 6;
    println!("{:?}", numeros);

    //Obtener el tamaño del arreglo
    println!("Longitud del arreglo: {}", numeros.len());

    //Obtener la cantidad de memoria que el arreglo toma (en el stack)
    println!("El arreglo ocupa {} bytes ", mem::size_of_val(&numeros));

    //Obteniendo pedazos del arreglo
    let pedazo: &[i32] = &numeros[0..3];
    println!("Pedazo: {:?}", pedazo);
}