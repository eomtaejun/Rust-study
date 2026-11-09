#[derive(Debug)]    // 컴파일러에게 전달하는 지시자 역할 (debug를 위한 무언가를 만들라는 요청)
struct Rectangle{
    width: u32,
    height: u32
}

fn main() {
    let rect=Rectangle{
        width: 20,
        height: 30
    };

    println!("area size is {}", area(&rect));
    println!("rect: {:?}", rect);   // {}=>display / {:?}=>debug 관련 메서드 호출
    dbg!(rect);                     // dbg macro: src file의 위치, line number, 구조체의 구체적인 구조 출력
}


fn area(rect: &Rectangle)->u32{
    rect.width*rect.height
}