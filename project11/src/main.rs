fn main() {
    let mut s = String::from("Olhou no espelho, o reflexo piscou.");

    let palavra = primeira_palavra(&s);

    // s.clear();

    println!("Primeira palavra: {}", palavra);
}

fn primeira_palavra(s: &String) -> &str {

    for (i, &item) in 
        s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn other_main() {
    let s = String::from("Olhou no espelho, o reflexo piscou.");

    let olhou = &s[0..5];
    let espelho = &s[6..12];
    let reflexo = &s[13..19];
    let piscou = &s[20..26];

    println!("{} {} {} {}", olhou, espelho, reflexo, piscou);
}

fn other_main_2() {

    let s = "Olhou no espelho, o reflexo piscou.";

    println!("{}",s);
}
