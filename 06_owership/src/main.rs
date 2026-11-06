fn main() {
    /*
        Rust의 메모리 관리: 컴파일 시점에 메모리 관리 규칙을 검사 (GarbageCollector 및 수동 관리 프로그램 언어와 큰 차이)
        - 소유권에 규칙에 따라 컴파일 시점에 메모리 할당/해제 관리
        - 규칙을 잘 지킨다면 컴파일러가 알아서 잘 처리해줌
        - 실행 시간에 손해가 없으면서, 실수할 여지가 없음 (Rust의 독자적 강점)
        - 규칙에 어긋나면 컴파일조차 X
     */

    // 변수 범위 scope가 끝나면 메모리 해제 가능
    {
        let s="hello";
        println!("s: {}", s);
    }
    // println!("s: {}", s);    <= 불가능

    /*
        stack and heap memory
        - stack: 나중에 넣을 값을 먼저 꺼내는 구조. 함수의 지역 데이터 등을 저장하며, 저장할 데이터으 크기를 컴파일 시점에 알아야 한다
        - heap: 실행 중 필요한 크기의 공간을 할당하며, 참조나 포인터로 접근한다
     */

    /*
        문자열 literal과 String 타입
        - "hello": 프로그램에 포함되는 수정 불가능한  문자열 데이터
        - let s="hello": s는 &str 타입으로, 해당 데이터의 위치와 길이를 가진다 (read only)
        - String::from("hello"): 내용을 heap에 복사해 직접 소유하는 문자열을 만든다 (mutable이기 때문에 컴파일 시점에 크기를 알 수 없음 => heap 메모리 사용)
        - String은 mutable로 선언하면 내용을 수정하거나 추가할 수 있다
     */
    let mut s: String=String::from("hello");
    s.push_str(", world!");

    /*
        ***소유권 규칙***
        - Rust에서 모든 값은 소유자(ower)가 있다
        - 한 시점에 하나의 소유자만 있을 수 있다
        - 소유자의 범위가 끝나면 값도 제거된다
     */
    {
        let s1=String::from("hello");   // heap 메모리에 위치
        println!("s1: {s1}");       // 사용 가능
        let s2=s1;                      // s2가 s1의 소유권을 가져감 (hello 값에 대한 위치, 길이, 용량 등의 정보가 s2로 넘어감 => s1은 사용 불가)
        println!("s2: {s2}");
        // println!("s1: {s1}");    // error
    }
    {
        let s1=String::from("hello");
        let s2=s1.clone();  // clone으로 heap 메모리 자체도 복사
        println!("s1: {s1}");
        println!("s2: {s2}");
    }
    {
        // 정수는 값이 copy됨 (String은 copy 타입이 아닌 move 타입임)
        let x=3;
        let y=x;
        println!("x: {x}");
        println!("y: {y}");
    }


    // 함수 호출 시 소유권 이동
    let s=String::from("hello");
    string_length(s);
    // println!("s: {s}");  // error
    let x=3;
    double(x);
    println!("x: {x}");     // 기본 데이터 타입은 이동이 아닌 복사이기 때문에 소유권을 잃지 않음
    

    // 함수 반환값의 소유권 이동
    let s=String::from("hello, world");
    let rtn=rtn_string(s);
    // println!("s: {s}");  // s의 소유권이 rtn_string으로 넘어감
    println!("s: {rtn}");   // 함수에서 값을 그대로 반환했기 때문에 소유권이 다시 rtn으로 넘어감


    // 불필요한 소유권 이동 문제
    let s=String::from("hello");
    let (len, s)=rtn_length(s);
    println!("string: {} / length: {}", s, len);
}

fn string_length(s: String){
    println!("string length: {}bytes", s.len());
}

fn double(x: i32){
    println!("x: {}", x*2);
}

fn rtn_string(s: String)->String{
    println!("The length of s is {}", s.len());
    s
}

fn rtn_length(s: String)->(usize, String){
    println!("s: {s}");
    (s.len(), s)
}