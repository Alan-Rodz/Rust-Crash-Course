//Las tuplas grupan diferentes valores
//Como maximo pueden tener 12 elementos

pub fn run()
{
    let persona:(&str, &str, i8) = ("Juan", "Perez", 37);
    println!("{} se apellida {} y tiene {} años", persona.0, persona.1, persona.2);
}
