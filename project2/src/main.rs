fn main() {
    
    const VELOCIDADE_DA_LUZ: u32 = 299_792_458;
    println!("{VELOCIDADE_DA_LUZ}");

    let mut imutavel = 10;
    println!("Valor: {imutavel}");
    imutavel = 20;
    println!("Novo valor: {imutavel}");

    let palavra = "Passarinho";

    println!("{}", palavra);

    println!("{:?}", palavra);
}
