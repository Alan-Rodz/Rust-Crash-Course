//Condicionales como en otros lenguajes
//Operadores: && = AND, || = OR,
//Rust no tiene operador ternario

pub fn run() {
    let edad: u8 = 18;
    let checar_id: bool = false;

    //If else y operadores
    if edad >= 21 && checar_id {
        println!("¿Qué quieres beber?");
    } else if edad < 21 && checar_id {
        println!("Eres demasiado joven para beber!");
    } else {
        println!("Necesito ver tu identificación");
    }

    //if pequeño
    let tiene_edad = if edad >= 21 { true } else { false };
    println!("Tiene edad : {}", tiene_edad);
}
