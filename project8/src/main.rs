fn main() {
    let cont = 10;
    let mut sum = 0;

    for i in 0..cont {
        sum += i;
    }

    println!("{}", sum);

    for i in 0..10 {
        println!("{}", i);
    }

    println!("{}", i);

    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }

    let mut i = 0;
    loop {
        println!("{}", i);
        i += 1;
        if i == 10 {
            break;
        }
    }
    
}
