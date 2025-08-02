// Программа перевода температуры по Фаренгейту в Цельсии
// Формула для перевода температуры из Фаренгейта в Цельсий: °C = 5/9(°F - 32)
use std::io;

fn main() {

    println!("Пожайлуста, введите температуру по Фаренгейту");
        
    let mut temperature_f = String::new();
        
    io::stdin().read_line(&mut temperature_f)
        .expect("Не получилось прочитать строку");

    let temperature_f: f32 = temperature_f.trim().parse()
        .expect("Не получилось");

    let temperature_c: f32 = (temperature_f - 32.0) * 5.0 / 9.0;
    println!("Температура по Фаренгейту {} в Цельсий: {}", temperature_f, temperature_c);

}
