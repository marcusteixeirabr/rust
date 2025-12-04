#[derive(Debug)] // Assim podemos verificar o Estado
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // --snip---
        }
    }
}

enum Coin {
Penny,
Nickel,
Dime,
Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} é bem antiga para a  América!"))
        } else {
            Some(format!("{state:?} é relativamente nova."))
        }
    } else {
        None
    }
}

fn main() {

    let mut carteira = 0u8;
    
    let moedas = [
    Coin::Nickel,
    Coin::Dime,
    Coin::Penny,
    Coin::Quarter(UsState::Alabama),
    Coin::Quarter(UsState::Alaska),
    ];

    for moeda in moedas.iter() {
        carteira += value_in_cents(moeda);
        /*
        match moeda {
            Coin::Quarter(state) => println!("Origem do quarter é: {state:?}"),
            _ => println!("Origem da moeda desconhecida!"),
        }
        */
        if let Coin::Quarter(state) = moeda {
            println!("Origem do quarter é: {state:?}");
        } else {
            println!("Origem da moeda desconhecida!");
        }

        let antiga = describe_state_quarter(moeda);
        
        println!("{antiga:?}");
    }



    println!("O valor da carteira é {carteira}");

    
}