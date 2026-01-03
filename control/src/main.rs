fn main() {
    // Выражения if
    // пример 1
    let number = 3;
    if number < 5 {
        println!("условие было истинным");
    } else {
        println!("условие было ложным");
    }

    // пример 2
    // let n = 3;
    // if n {
    //     println!("число было равно трем");
    // }
    // ошибка ожидается тип bool

    // пример 3
    let n = 3;
    if n != 0 {
        println!("число было отличным от нуля");
    }

    // Обработка нескольких условий с помощью else if
    let n2 = 6;
    if n2 % 4 == 0 {
        println!("число делится на 4");
    } else if n2 % 3 == 0 {
        println!("число делится на 3");
    } else if n2 % 2 == 0 {
        println!("число делится на 2");
    } else {
        println!("число не делится на 4, 3 и 2");
    }

    // Использование выражения if в инструкции let
    let condition = true;
    let m = if condition {
        5
    } 
    else {
        6
    };
    println!("значение числа равно {}", m);

}
