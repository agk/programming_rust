enum Message {
    Qiut,
    Move { x : i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct QuitMesssage; // пустая структура
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // кортежная структура
struct ChangeColorMessage(i32, i32, i32); //кортежная структура

impl Message {
    fn call(&self) {
        // здесь будет определено тело метода
    }
}

let m = Message::Write(String::from("hello"));
m.call();

fn main() {

}