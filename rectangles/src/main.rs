fn main() {
    let rect1 = (30, 50);

    println!(
        "Площадь прямоугольника равна {} квадратным пискселам.",
        area(rect1)
    );


}

fn area(demensions: (u32, u32)) -> u32 {
    demensions.0 * demensions.1
}
