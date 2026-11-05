fn main() {
    // 변수 선언: let
    // mut 키워드 사용 시 변할 수 있는 변수
    let mut a=1;

    // 상수 선언: const
    // 무조건 대문자로 identify 지정 및 타입 명시
    const PI: f32=3.141592; // 부동소수점 32비트

    // shadowing (변수 가리기)
    let x=3;
    println!("1) x: {x}");      // 3

    let x=x+1;
    println!("2) x: {x}");      // 4

    {   // 스코프 차이
        let x=x*2;
        println!("3) x: {x}");  // 8
    }

    println!("4) x: {x}");      // 4
}
