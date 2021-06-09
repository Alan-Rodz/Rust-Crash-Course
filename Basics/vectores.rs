//Los vectores son arreglos de tamaño variable

use std::mem;

pub fn run()
{
    let mut numeros: Vec<i32> = vec![1, 2, 3, 4];

    //Añadiendo al vector
    numeros.push(20);
    numeros.push(22);

    //Removiendo del vector (el ultimo elemento)
    numeros.pop();

    //Imprimiendo todo el vector
    println!("{:?}", numeros);

    //Obteniendo un valor particular
    println!("{}", numeros[0]);

    //Obtener el tamaño del vector
    println!("Longitud del vector: {}", numeros.len());

    //Obtener la cantidad de memoria que el arreglo toma (en el stack)
    println!("El vector ocupa {} bytes ", mem::size_of_val(&numeros));

    //Obteniendo pedazos del arreglo
    let pedazo: &[i32] = &numeros[0..3];
    println!("Pedazo: {:?}", pedazo);

    //Iterando a través del vector
    for x in numeros.iter()
    {
        println!("Numero: {}", x)
    }

    //Iterando y mutando valores del vector
    for x in numeros.iter_mut()
    {
        *x *= 2;
    }

    println!("Vector de numeros: {:?}", numeros);
}
