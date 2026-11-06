fn main() {
    // 소유권 빌리기(borrowing)는 소유권을 넘기지 않고 참조(&T or &mut T)를 통해 데이터에 접근하는 것이다
    // &String은 String의 관리를 정보를 가리키고, &str은 문자열 데이터의 주소와 길이를 가진다
    let s=String::from("hello");

    let len=calc_length(&s);

    println!("The length of {} is {}", s, len);

    /*
        reference (참조)
        - 소유권을 넘기지 않고 데이터에 접근
        - 하나의 mut 참조가 있거나, immutable 참조만 여럿 있으면 활용 가능
        
        - 특정 데이터가 위치한, 접근할 수 있는 주소
        - 해당 데이터는 기존의 다른 소유자가 보유
        - 임대는 기본적으로 immutable
     */
    let mut s=String::from("hello");

    append_word(&mut s);
    println!("s: {s}");

    // 임대는 여러 번 변경 불가능 (&mut 여러 번 X)
    let mut s=String::from("hello");

    /*
        데이터 경쟁 조건 (data race)
        - 둘 이상의 포인터가 같은 데이터를 참조
        - 한 개 이상의 포인터가 데이터를 쓰려고 접근
        - 해당 데이터 접근을 동기화할 방법이 없음
        => Rust는 컴파일에서 데이터 경쟁 조건을 방지
     */
    {
        let r1=&mut s;  // mut 참조 1
        let r2=&mut s;  // mut 참조 2
        // println!("r1: {} / r2: {}", &r1, &r2);  // 동시에 참조 => data race => error
    }
    {
        let r1=&s;          // immutable 참조 1
        let r2=&s;          // immutable 참조 2
        let r3=&mut s;  // mut 참조
        // println!("r1: {} / r2: {} / r3: {}", r1, r2, r3);   // 참조 여러 번 + &mut => 불가능
    }
    {
        let r1=&s;          // immutable 참조 1
        let r2=&s;          // immutable 참조 2
        println!("r1: {} / r2: {}", r1, r2);    // 참조의 범위는 변수의 실제 사용되는 부분까지 검사
        let r3=&mut s;  // mut 참조
        println!("r3: {}", r3);     // 참조 여러 번 + &mut (범위가 겹치지 않으면) => 가능
    }

    /*

     */
}

fn calc_length(s: &String)->usize{
    s.len()
}

fn append_word(s: &mut String){
    s.push_str(", world");
}