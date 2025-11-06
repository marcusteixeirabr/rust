use std::process::Command;

fn main() {

    let mut keyboard = String::new();

    limpa_tela();

    println!("{}", "-=".repeat(12));
    println!("Conversor de Temperatura");
    println!("{}", "-=".repeat(12));

    println!("Escolha:");
    println!("[1] Celsius para Fahrenheit");
    println!("[2] Fahrenheit para Celsius");

    let unit = loop {
        keyboard.clear();
        std::io::stdin()
            .read_line(&mut keyboard)
            .expect("Falha na leitura de dados");
        if let Ok(val) = keyboard.trim().parse::<u32>() {
            if val == 1 {
                println!("Vamos converter ºC em ºF: ");
                break val
            } else if val == 2 {
                println!("Vamos converter ºF em ºC: ");
                break val
            }
        }
        println!("Digite apenas um dos valores.");
    };

    println!("Digite a temperatura.");
    let temperature = loop {
        keyboard.clear();
        std::io::stdin()
            .read_line(&mut keyboard)
            .expect("Falha na leitura de dados");
        if let Ok(val) = keyboard.trim().parse::<f32>() {
            break val
        }
        println!("Digite apenas números.");
    };

    temperature_converter(temperature, unit);
}


fn limpa_tela() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn temperature_converter(temperature: f32, unit: u32) {
    if unit == 1 {
        println!("{temperature}ºC equivale a {:.1}ºF", temperature * 1.8 + 32.0);
    } else if unit == 2 {
        println!("{temperature}ºF equivale a {:.1}ºC", (temperature - 32.0) / 1.8);
    }
}
