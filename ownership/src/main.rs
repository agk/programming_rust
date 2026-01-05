fn main() {

    // Область видимости переменной
    {
        // s здесь не действует; она еще не объявлена
        let _s = "hello "; // s действует с этого момента и далее
        // что-то сделать с s
    } // эта область закончилась, и s больше не действует

    // Строковый тип
    // let s = String::from("hello");
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() добавляет литерал к экземпляру типа String
    println!("{}", s); // эта инструкция выводит `hello, world!

    // Память и выделение пространства
    {
        let _s = String::from("hello"); // s действительна с этого момента и далее
        // что-то сделать с s
    }   // эта область закончилась, и
    // s больше не действует

    // Способы взаимодействия переменных и данных: перемещение
    let _x = 5;
    let _y = _x;

    let s1 = String::from("hello");
    let _s2 = s1;
    // println!("{}, world!", s1); // ошибка s1 в памяти уже не существует

    // Пути взаимодействия переменных и данных: Clone
    let str1 = String::from("hello");
    let str2 = str1.clone();
    println!("srt1 = {}, str2 = {}", str1, str2);

    // Данные только из стека: Copy
    let a = 5;
    let b = a;
    println!("a = {}, b = {}", a, b);

    // Владение и функции
    let str_new = String::from("hello");    // s входит в область видимости
    takes_ownership(str_new);               // значение s перемещается в функцию...
                                            // ... и поэтому больше здесь не действует
    let xn = 5;         // x входит в область видимости
    makes_copy(xn);     // x переместится в функцию, но
                        // i32 копируема, поэтому нормально,
                        // если x будет использоваться после этого

    // Возвращаемые значения и область видимости
    let _s1 = gives_ownership();         // gives_ownership перемещает свое возвращаемое значение в s1
    let s2 = String::from("hello");     // s2 входит в область видимости
    let _s3 = takes_and_gives_back(s2);  // s2 перемещается в takes_and_gives_back, 
                                        // которая также перемещает свое возвращаемое значение в s3
    
    // С помощью кортежа можно возвращать несколько значений
    let str_n1 = String::from("hello");
    let (str_n2, len) = calculate_length(str_n1);
    println!("Длина '{}' равна {}.", str_n2, len);
    


}   // Здесь x выходит из области видимости, а затем s. 
    // Но поскольку значение s было перемещено, ничего особенного не происходит.

// Здесь s3 выходит из области видимости и отбрасывается. s2 выходит
// из области видимости, но была перемещена, поэтому ничего не происходит.
// s1 выходит из области и отбрасывается.

fn gives_ownership() -> String {    // gives_ownership переместит свое
                                    // возвращаемое значение в функцию,
                                    // которая ее вызывает
    let some_string = String::from("hello");    // some_string входит
                                                // в область видимости 
    some_string     // some_string возвращается и
                    // выносится в вызывающую функцию
}

// takes_and_gives_back возьмет String и вернет String
fn takes_and_gives_back(a_string: String) -> String {   // a_string приходит
                                                        // в область видимости
    a_string    // a_string возвращается и выносится в вызывающую функцию
}

fn takes_ownership(some_string: String) {   // some_string входит
                                            // в область видимости
    println!("{}", some_string);
}   // Здесь some_string выходит из области видимости и вызывается `drop`.
    // Поддерживающая память высвобождается.

fn makes_copy(some_integer: i32) {  // some_integer входит в область видимости
    println!("{}", some_integer);
}   // Здесь some_integer выходит из области видимости.
    // Ничего особенного не происходит.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() возвращает длину экземпляра типа String
    (s, length)
}