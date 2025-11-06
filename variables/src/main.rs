fn main() {
    let mut x = 5.0;
    println!("O valor de x é: {x}"); // 5
    x = 6.0;
    println!("O valor de x é: {x}"); // 6

    let y = 5.0;
    println!("O valor de y é: {y}"); // 5
    let y = y + 1.0;
    println!("O valor de y é: {y}"); // 6
    {
        let y = y * 2.0;
        println!("O valor de y é: {y}"); // 12
    }
    println!("O valor de y é: {y}"); // 6

    let space = "    "; // "    "
    println!("O valor de space: {space}");
    let space = space.len(); // 4
    println!("O valor de space: {space}");

    const NUMERO_PI: f32 = 3.14159265358979323846264338327950288;
    println!("O valor de NUMERO PI: {NUMERO_PI}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("{a} {b} {c}");

    let tup = (500.5, 6, 'c', 1);
    let (a, b, c, d) = tup;
    println!("{a} {b} {c} {d}");

    let tup = (x, y, space);
    let (a, b, c) = tup;
    println!("{a} {b} {c}");

    let five_hundred = tup.0;
    let six = tup.1;
    println!("five hundred {five_hundred}");
    println!("six {six}");
    println!("tup {tup:?}");

    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("arr {arr:?}");
    let first = arr[0];
    println!("first: {}", first);

    println!("Entre com o índice da matriz arr: ");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Falha ao ler a linha!");

    let index: usize = index
        .trim()
        .parse()
        .expect("O índice digitado não é um número!");

    let element = arr[index];
    println!("O valor de element no índice {index} é: {}", element);




}
