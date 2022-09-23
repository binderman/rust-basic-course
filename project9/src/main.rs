fn old_main() {

    for _ in 0..3 {
        let s = String::from("Rust");
        println!("{}", s);
    }

    for i in 0..3 {
        let s = String::from(format!("{}", i));
        println!("{} está no endereço {:p}", s, &s);
    }

    let x = 42;
    let y = x;
    println!("{}", y);
    println!("{}", x);
    
    let s1 = String::from("Rust");
    let s2 = s1;
    println!("{}", s2);
    // println!("{}", s1);

    let s1 = String::from("Rust");
    let s2 = s1.clone();
    println!("{}", s2);
    println!("{}", s1);
}

fn old_old_main() {
    let x = 42;
    copy_scalar(x);
    println!("x é válido: {}", x);
}

fn copy_scalar(n: i32) {
    println!("{}", n);
}

fn main() {
    let s = String::from("Rust");
    move_string(&s);
    println!("{}", s);
}

fn move_string(r: &String) {
    println!("{}", r);
}
