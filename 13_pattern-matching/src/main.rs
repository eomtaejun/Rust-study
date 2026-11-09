enum Color{
    Red,
    Green,
    Blue
}

enum Message{
    StartGame,
    WinPoint{who: String},
    ChangePlayerName(String)
}

struct RGB(u8, u8, u8);

fn main() {
    /*
        match 구문
        - 구분 가능한 값에 따라 처리 (switch/case문과 유사)
        - match 시 해당 값을 꺼낼 수 있음
        - Enum이 가지고 있는 모든 경우에 대해 처리해야하는 제약 사항 (Exhaustive)
     */

    let x=Some(2);
    println!("{:?}", increment(x));     // Some(3)
    println!("{:?}", increment(None));  // None
}

fn color_to_rgb(color: Color)->RGB{
    // 반드시 모든 경우에 대해 처리 (Exhaustive)
    match color{
        Color::Red=>RGB(255, 0, 0),
        Color::Green=>RGB(0, 255, 0),
        Color::Blue=>RGB(0, 0, 255),
    }
}

fn handle_message(message: &Message){
    match message{
        Message::StartGame=>println!("game start"),
        Message::WinPoint{who}=>println!("{who}의 득점"),
        // Message::ChangePlayerName(name)=>println!("change player name: {name}")
        // _=>println!("message not yet implemented")  // match 구문의 와일드카드 _
        Message::ChangePlayerName(_)=>println!("request to change player name")  // match 구문의 와일드카드 _
    };
}

fn increment(x: Option<i32>)->Option<i32>{
    match x{
        Some(i)=>Some(i+1),
        None=>None
    }
}