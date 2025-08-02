fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("число делится на 4");
    }
    else if number % 3 == 0 {
        println!("число делится на 3");
    }
    else if number % 2 == 0 {
        println!("число делится на 2");
    }
    else {
        println!("число не делится на 4, 3 и 2");
    }
}
