#![allow(dead_code)]
#![allow(unused_variables)]


fn main() {
    let numero_existente = Some(10);
    let char_existente = Some('a');

    let numero_inexistente: Option<i32> = None;

    let numero = 42;
    let maybe_numero = Some(42);

    let resultado = match maybe_numero {
        Some(n) => n,
        None => 0,
    };

    // println!("{:?}", numero + maybe_numero);  // Erro!
}

#[derive(Debug)]
enum Crypto {
    Bitcoin(String),
    Ethereum,
    Cardano,
    Monero,
    Dogecoin,
}

fn network(crypto: Crypto) {
    match crypto {
        Crypto::Bitcoin(network) => println!("Bitcoin network: {}", network),
        _ => println!("mainnet"),
    }
}

fn network_(crypto: Crypto) {
    if let Some(network) = Some(crypto) {
        println!("Bitcoin network: {:?}", network);
    } else {
        println!("mainnet");
    }
}

fn decimals_(crypto: Crypto) -> u8 {
    match crypto {
        Crypto::Bitcoin(_) => 8,
        Crypto::Ethereum => 18,
        Crypto::Cardano => 6,
        Crypto::Monero => 12,
        _ => 0,
    }
}

fn decimals(crypto: Crypto) -> u8 {
    match crypto {
        Crypto::Bitcoin(_) => 8,
        Crypto::Ethereum => 18,
        Crypto::Cardano => 6,
        Crypto::Monero => 12,
        other => {
            println!("Desconhecido: {:?}", other);
            0
        }
    }
}

enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

fn digits(base: Base) -> u32 {
    match base {
        Base::Binary => 2,
        Base::Octal => 8,
        Base::Decimal => 10,
        Base::Hexadecimal => 16,
    }
}

fn convert(num: i32, base: Base) -> String {
    match base {
        Base::Binary => format!("{:b}", num),
        Base::Octal => format!("{:o}", num),
        Base::Decimal => format!("{}", num),
        Base::Hexadecimal => format!("{:x}", num),
    }
}