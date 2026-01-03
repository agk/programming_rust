fn main() {
    // пример loop бесконечного
    // loop {
    //     println!("еще раз!");
    // }

    // пример 
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Результат равен {}", result);

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("Поехали!!!");

    // Осуществление цикла в коллекции с помощью for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("Значение равно {}", a[index]);
        index = index + 1;
    }

    // Осуществление цикла в элементах коллекции с помощью for
    // let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Значение равно {}", element);
    }

    // Вот как выглядит обратный отсчет с использованием цикла for 
    // и еще одного метода, о котором мы пока не говорили, 
    // — rev — для инвертирования интервала 
    for element2 in (1..4).rev() {
        println!("{}!", element2);

    }
    println!("Поехали!!!")
    
}
