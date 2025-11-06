use std::io;

fn main() {
    let mut entrada= String::new();
    let mut region: usize = 0;
    const FUEL_PRICE: f32 = 5.90;

    loop {
        println!("Deseja rastrear o frete? [S/N]");
        entrada.clear();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro ao ler a resposta");
        match entrada.trim().to_lowercase().as_str() {
            "s" | "n" => break,
            _ => continue,
        }
    }
    let track: f32 = if entrada.trim().to_lowercase() == "s" { 200.0 } else { 0.0 };

    let distance: f32 = ask_number("Qual a distância do frete em Km?");
    let parts: f32 = ask_number("Digite a quantidade de peças");
    while region < 1 || region > 3 {
        region = ask_number("Digite a região destino:\n1 - Sul\n2 - Sudeste\n3 - Centro-Oeste") as usize;
    }
    let parts_freight: f32 = calculate_parts_freight(region, parts);
    let distance_freight: f32 = distance * FUEL_PRICE;

    println!("Taxa do rastreamento: R$ {track:.2}");
    println!("Valor do frete pelas peças: R$ {parts_freight:.2}");
    println!("Valor do frete por Km: R$ {distance_freight:.2}");
    println!("Total do frete: R$ {:.2}", track + parts_freight + distance_freight);
}

fn ask_number(pergunta: &str) -> f32 {
    let mut ask = String::new();
    println!("{}", pergunta);
    loop {
        ask.clear();
        io::stdin()
            .read_line(&mut ask)
            .expect("Erro ao ler a resposta");
        if let Ok(num) = ask.trim().parse::<f32>() {
            if num > 0.0 {
                return num;
            }
        }
        println!("Digite uma valor válido!");
    }
}

fn calculate_parts_freight(region: usize, parts: f32) -> f32 {
    let region = region -1;
    let freight = [1.0, 1.2, 1.3];
    let discount_tax = [10.0, 12.0, 13.0];
    parts * freight[region] - if parts > 1000.0 {(parts - 1000.0) * freight[region] * (discount_tax[region]) / 100.0} else {0.0}
}