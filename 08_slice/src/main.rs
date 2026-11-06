fn main() {
    /*
        slice 타입
        - 어떠한 모음 요소들 중에 일부 연속된 요소들만 참조하는 방법
        - reference와 마찬가지로 소유권을 넘기지 않음
     */

    
    let s=String::from("hello, world!");
    let word=&s[0..5];
    println!("word: {word}");
    let word: &str=&s[7..13];
    println!("word: {word}");

    // 문자열 slice 범위 표현
    let len=s.len();
    let hello=&s[0..5];
    let hello=&s[..5];
    let world=&s[7..13];
    let world=&s[7..];
    let world=&s[7..len];
    let all=&s[..];
    println!("hello: {} / world: {} / all: {}", hello, world, all);

    // slice 및 reference 실습
    let s=String::from("hello, world!");
    let first=first_word(&s);
    println!("first word: {first}");

    // &str 외의 다른 슬라이스
    let arr=[1, 2, 3, 4, 5];
    let slice=&arr[1..3];
    println!("arr: {:?} / slice: {:?}", arr, slice);
}

fn first_word(s: &String)->&str{
    let bytes=s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[..i];
        }
    }
    &s[..]
}