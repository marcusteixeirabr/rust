

fn main() {
	let is_logado = true;
	let is_admin = false;

	println!("É um usuário regular? {}", is_logado && !is_admin);
	println!("Tem acesso? {}", is_logado || is_admin);
	println!("Não está logado? {}", !is_logado);
	let age = 20;
	let can_vote = age >= 18;
	println!("Pode votar? {}", can_vote);
	let time = 20;
	let greeting = if time < 18 {
		"Good day."
	} else {
		"Good evening."
	};
	println!("{}", greeting);
	let day = 6;
	match day {
		1 => println!("Domingo"),
		2 => println!("Segunda"),
		3 => println!("Terça"),
		4 => println!("Quarta"),
		5 => println!("Quinta"),
		6 => println!("Sexta"),
		7 => println!("Sábado"),
		_ => println!("Dia inválido"),
	}
	match day {
		2 | 3 | 4 | 5 | 6 => println!("Dia de semana"),
		1 | 7 => println!("Fim de semana"),
		_ => println!("Dia inválido"),
	}
	let result = match day {
		1 => "Domingo",
		2 => "Segunda",
		3 => "Terça",
		4 => "Quarta",
		5 => "Quinta",
		6 => "Sexta",
		7 => "Sábado",
		_ => "Dia inválido",
	};
	println!("{}", result);
	let mut count = 1;
	loop {
		println!("{} repetição", count);
	count += 1;
	if count > 5 {
		break;
	}
	}
	let loops = loop {
		println!("{}", count);
		count += 1;
		if count >= 10 {
			break count * 2;
		}
	};
	println!("{}", loops);
	for i in 1..=10 {
		if i == 3 {
			continue;
		}
		if i == 5 {
			break;
		}
		println!("i é {}", i);
		let sum = somar(5, 6);
		println!("A soma é {}", sum);
	}
}

fn somar(a: i32, b: i32) -> i32 {
	a + b
}