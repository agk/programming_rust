// Программа перевода температуры по Цельсии в Фаренгейт 
// Формула для перевода температуры из градусов Цельсия в градусы Фаренгейта: 
// Фаренгейт (°F) = (Цельсий (°C) × 9/5) + 32. 
use std::io;

fn main() {

    println!("Пожайлуста, введите температуру по Цельсию:");
        
    let mut temperature_c = String::new();
        
    io::stdin().read_line(&mut temperature_c)
        .expect("Не получилось прочитать строку");

    let temperature_c: f32 = temperature_c.trim().parse()
        .expect("Не получилось");

    let temperature_f: f32 = (temperature_c * 9.0 / 5.0) + 32.0;
    println!("Температура по Цельсий {} в Фаренгейту: {}", temperature_c, temperature_f);

}
