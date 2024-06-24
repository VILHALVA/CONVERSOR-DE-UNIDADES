use std::io;

fn main() {
    loop {
        println!("Conversor de Unidades");
        println!("1. Quilômetros para Milhas");
        println!("2. Milhas para Quilômetros");
        println!("3. Celsius para Fahrenheit");
        println!("4. Fahrenheit para Celsius");
        println!("5. Sair");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => convert_km_to_miles(),
            2 => convert_miles_to_km(),
            3 => convert_celsius_to_fahrenheit(),
            4 => convert_fahrenheit_to_celsius(),
            5 => break,
            _ => println!("Escolha inválida"),
        }
    }
}

fn convert_km_to_miles() {
    println!("Digite o valor em quilômetros:");
    let km = read_input();
    let miles = km * 0.621371;
    println!("{} km é igual a {} milhas", km, miles);
}

fn convert_miles_to_km() {
    println!("Digite o valor em milhas:");
    let miles = read_input();
    let km = miles / 0.621371;
    println!("{} milhas é igual a {} km", miles, km);
}

fn convert_celsius_to_fahrenheit() {
    println!("Digite o valor em Celsius:");
    let celsius = read_input();
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    println!("{}°C é igual a {}°F", celsius, fahrenheit);
}

fn convert_fahrenheit_to_celsius() {
    println!("Digite o valor em Fahrenheit:");
    let fahrenheit = read_input();
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{}°F é igual a {}°C", fahrenheit, celsius);
}

fn read_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0.0)
}
