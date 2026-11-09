#[derive(Debug, PartialEq)]
enum Color{
    Red,
    Green,
    Blue
}

enum Message{
    StartGame,
    WinPoint{who: String},      // struct와 동일한 format
    ChangePlayerName(String)    // tuple과 동일한 format
}

// Rust 기본 내장 Enum
// enum Option<T>{
//     None,
//     Some(T)
// }

fn main() {
    // Enum: 여러 정해진 가짓 수 중 택일 값을 다루는 타입
    let red=Color::Red;
    let green=Color::Green;

    println!("red: {:?}", red);
    println!("red==green=>{}", red==green); // PartialEq: 자동 비교 연산
    println!("red==red=>{}", red==Color::Red); // PartialEq: 자동 비교 연산

    let m1=Message::StartGame;
    let m2=Message::WinPoint{who: String::from("user1")};
    let m3=Message::ChangePlayerName(String::from("user2"));

    let some_number=Some(2);
    let absent_number: Option<i32>=None;

    // some_number+1;   // Option+int는 불가능
}
