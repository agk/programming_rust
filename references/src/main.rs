fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    
    println!("Длина '{}' равна {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    // Срезовый тип
    let mut s2 = String::from("hello world");
    let _word = first_word(&s2); // переменная word получит значение 5
    s2.clear(); // это опустошает экземпляр типа String, делая его равным ""
    // здесь word по-прежнему имеет значение 5, но больше нет строки,
    // с которой мы могли бы осмысленно использовать значение 5.
    // переменная word теперь полностью недействительна!

    // Строковые срезы
    let _s3 = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];

}

fn calculate_length(s: &String) -> usize { // s — это ссылка на экземпляр
    // типа String
    s.len()
}   // Здесь s выходит из области видимости. Но поскольку она не владеет тем,
    // на что она ссылается, ничего не происходит.


fn change(some_string: &mut String) {
    some_string.push_str(", world");
    // Ссылки, точно так же, как и переменные, по умолчанию неизменяемы. 
    // Мы не можем модифицировать то, на что у нас есть ссылка
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}