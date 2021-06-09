//Usamos structs para crear tipos de informacion personalizada

//Struct tradicional
struct Color 
{
    rojo: u8,
    verde: u8,
    azul: u8,
}

//Struct tipo tupla
struct ColorTupla(u8, u8, u8);

//Struct con funciones
struct Persona 
{
    nombre: String,
    apellido: String,
}

//impl significa implements
impl Persona 
{
    //Constructor para una nueva persona
    fn nueva_persona(nom: &str, ap: &str) -> Persona 
    {
        Persona 
        {
            nombre: nom.to_string(),
            apellido: ap.to_string(),
        }
    }

    //Funcion geter para el nombre completo (self es como this)
    fn nombre_completo(&self) -> String 
    {
        format!("{} {}", self.nombre, self.apellido)
    }

    //Setter para el apellido
    fn modificar_apellido(&mut self, nuevo_apellido: &str) 
    {
        self.apellido = nuevo_apellido.to_string();
    }

    //Funcion que muestra el nombre completo como una tupla
    fn a_tupla(self) -> (String, String) 
    {
        (self.nombre, self.apellido)
    }
}

pub fn run() 
{
    let mut color1 = Color 
    {
        rojo: 255,
        verde: 0,
        azul: 0,
    };

    color1.rojo = 200;
    println!("Color1: {} {} {} ", color1.rojo, color1.verde, color1.azul);

    let mut color2 = ColorTupla(255, 0, 255);
    color2.0 = 255;
    println!("Color2: {} {} {}", color2.0, color2.1, color2.2);

    let mut p = Persona::nueva_persona("Juan", "Perez");
    println!("Persona: {}{}", p.nombre, p.apellido);
    println!("El nombre completo de p es: {}", p.nombre_completo());

    println!("Cambiando el apellido de p...");
    p.modificar_apellido("Gutierrez");
    println!("El nuevo nombre completo de p es: {}", p.nombre_completo());

    println!("Tupla Persona: {:?}", p.a_tupla());
}
