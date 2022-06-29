enum Message {
    Quit,                       // Quit has no data associated with it at all.
    Move { x: i32, y: i32 },    // Move has named fields like a struct does.
    Write(String),              // Write includes a single String.
    ChangeColor(i32, i32, i32), // ChangeColor includes three i32 values.
}

// The "struct" way to mimic the above would look something like this
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// Not very elegant due to we have to use the struct keyword everytime
// but we see a repeating pattern and usage of "..Message".
// Another disadvantage is that all structs a considered
// different data types, where enums are one and the same data type.

// Enum Message can also be "implemented"
impl Message {
    fn call(&self) {
        // method body would be difined here
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
