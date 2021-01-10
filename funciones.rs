
pub fn run()
{
    saludo("Hola", "Juan");
    
    let obtener_suma = suma(5,5);
    println!("{}", obtener_suma);

    //Tambien podemos definir funciones así
    //Estas manera nos permite acceder a variables que no están en el scope de la declaracion de la funcion
    let n3: i32 = 10;
    let agregar_numeros = |n1:i32, n2:i32| n1 + n2 + n3;
    println!("Suma: {}", agregar_numeros(3,3));
}

fn saludo(saludo: &str, nombre: &str)
{
    println!("{} {}, que gusto conocerte!", saludo, nombre);
}

//Funcion con retorno
fn suma(n1:i32, n2:i32) -> i32 
{
    //Cuando no usamos ; indicamos que eso es lo que queremos regresar
    n1 + n2
}