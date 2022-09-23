#[derive(Debug)]
struct Cliente {
    nome: String,
    idade: u8,
    ativo: bool,
    saldo: f64,
}

impl Cliente {
    // Self é o tipo do impl, no caso Cliente
    fn new(nome: String, idade: u8, ativo: bool, saldo: f64) -> Self {
        Self {
            nome,
            idade,
            ativo,
            saldo,
        }
    }

    // O método altera a struct, por isso 
    // self precisa ser mutável
    fn depositar(&mut self, valor: f64) -> f64 {
        self.saldo += valor;

        self.saldo
    }
}

fn main() {
    let mut cli = Cliente {
        nome: "Bruce".to_string(),
        idade: 35,
        ativo: true,
        saldo: 100_000_000.0,
    };

    let cli2 = Cliente::new("Steve".to_string(), 
                                        70, 
                                        true, 
                                        10_000.0);
    
    println!("{:?}", cli2);

    cli.depositar(1_000_000.0);
    println!("Saldo: {:.2}", cli.saldo);

    println!("{:?}", cli.nome);
    println!("{:?}", cli.idade);
    println!("{:?}", cli.ativo);
}


fn old_main() {
    let mut cli = Cliente {
        nome: "Zeca".to_string(),
        idade: 30,
        ativo: true,
        saldo: 100.0,
    };

    cli.saldo += 50.0;

    println!("{}", cli.nome);
    println!("{}", cli.idade);
    println!("{}", cli.ativo);
    println!("{}", cli.saldo);

    abrir_conta("Zeca".to_string(), 30);

    let cli2 = Cliente {
        nome: "Maria".to_string(),
        ..cli
    };

    println!("{}", cli2.nome);
}

fn abrir_conta(nome: String, idade: u8) -> Cliente {
    Cliente {
        nome,
        idade,
        ativo: true,
        saldo: 0.0,
    }
}

#[derive(Debug)]
struct GlobalLocation(u64, u64);

fn old_old_main() {
    let mut gl = GlobalLocation(0, 0);
    gl.0 += 1;
    gl.1 += 1;

    println!("{:?}", gl);
    dbg!(gl);
}

