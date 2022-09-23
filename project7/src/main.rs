use std::io::stdin;

fn main() {
    loop {
        let mut number = String::new();
        println!("Digite um número: ");
        stdin().read_line(&mut number).expect("Erro na leitura");
        println!("Você digitou: {}", number);

        let number = match number.trim().parse::<u32>() {
            Ok(num) => num,
            Err(e) => {
                println!("Erro: {}", e);
                continue;
            }
        };

        let result = if number % 2 == 0 { "par" } else { "impar" };
        println!("{} é {}", number, result);

        break;
    }
}
