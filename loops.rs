pub fn run()
{
    //Loop infinito
    let mut cuenta = 0;
    loop 
    {
        cuenta += 1;
        println!("Numero {}", cuenta);

        if cuenta == 20
        {
            break;
        }
    }

    //While fizzbuzz
    cuenta = 0;
    while cuenta <= 100 
    {
        if cuenta % 5 == 0 && cuenta % 3 == 0
        {
            println!("Fizzbuzz!");
        }
        else if cuenta % 3 == 0
        {
            println!("Fizz!");
        }
        else if cuenta % 5 == 0
        {
            println!("Buzz!");
        }
        else
        {
            println!("{}", cuenta);
        }

        cuenta += 1;
    }

    //For con rango
    for x in 0..100 
    {
        println!("{}", x);
    }
}