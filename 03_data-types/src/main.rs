fn main() {
    // Rust에서 모든 값은 특정 데이터 타입의 값
    // Scalar(기본값) OR Compound(복합값)
    // Rust는 정적 타입의 언어
    
    // #region Scalar Type
    // 정수 integer 타입
    /*
        | 크기     | 부호 있음 | 부호 없음 |
        |----------|-----------|-----------|
        | 8비트    | i8        | u8        |
        | 16비트   | i16       | u16       |
        | 32비트   | i32       | u32       |
        | 64비트   | i64       | u64       |
        | 128비트  | i128      | u128      |
        | 아키텍처 | isize     | usize     |
    */

    // 정수 리터럴 표현
    /*
        | 표기 방식   | 예            |
        |-------------|---------------|
        | 10진수      | 19_384        |
        | 16진수      | 0xff          |
        | 8진수       | 0o77          |
        | 2진수       | 0b1111_0010   |
        | 바이트 (u8) | b'A'          |
    */

    // 부동소수점 floating-point 타입
    /*
        | 크기   | 부호 있음 |
        |--------|-----------|
        | 32비트 | f32       |
        | 64비트 | f64       |
    */

    // 숫자 기본 연산
    let add=3+8;
    let sub=26.5-2.1;
    let mul=7*20;
    let quotient=12.0/3.14;
    let truncated=7/5;  // 결과는 1
    let remainder=46%5;

    // boolean 타입
    let t=true;
    let f: bool=false;  // boolean 명시

    // 문자 타입 (Unicode 형태여서 한글 및 이모지 가능)
    let c='A';
    let z: char='가';
    let unicorn='🦄';
    // #endregion Scalar Type


    // #region Compound Type
    // tuple 타입
    let t: (i32, bool, f64)=(32, true, 1.41);
    let (x, y, z)=t;    // 구조분해기법
    println!("x: {x} / y: {y} / z: {z}");

    let t=(32, true, 1.41);
    let x=t.0;  // index 접근
    let y=t.1;  // index 접근
    let z=t.2;  // index 접근

    // unit 타입: 빈 튜플
    let unit: ()=();    // 반환값 없음을 의미 (c의 void와 유사)

    // array 타입
    let arr: [i32; 5]=[1, 2, 3, 4, 5];  // 배열은 동일한 타입만, 길이 고정
    // arr[0]; => 다른 언어와 동일하게 index 활용
    // #endregion Compound Type
    let threes=[3; 100];    // a값으로 n번 채운 배열 생성
}
