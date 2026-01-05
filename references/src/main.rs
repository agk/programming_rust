fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    
    println!("Длина '{}' равна {}.", s1, len);

    let s = String::from("hello");
    change(&mut s);

}

fn calculate_length(s: &String) -> usize { // s — это ссылка на экземпляр
    // типа String
    s.len()
}   // Здесь s выходит из области видимости. Но поскольку она не владеет тем,
    // на что она ссылается, ничего не происходит.


fn change(some_string: &String) {
    some_string.push_str(", world");
    // Ссылки, точно так же, как и переменные, по умолчанию неизменяемы. 
    // Мы не можем модифицировать то, на что у нас есть ссылка
}