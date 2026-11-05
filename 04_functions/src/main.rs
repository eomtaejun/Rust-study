use std::f32::consts::LN_10;

fn main() {
    println!("Hello, world!");

    a_function();
    add_numbers(2, 3);

    let x={
        let y=3;
        y+5 // 마지막 세미콜론 없는 값이 블록 전체의 값이 됨
    };
    println!("expression x: {x}");  // 반환되는 값이 존재 (식)

    let area=circle_area(2.0);
    println!("area: {area}");
}

fn a_function(){
    println!("It's a defferent function");
}

// function parameters
fn add_numbers(x: i32, y: i32){
    let sum=x+y;
    println!("x+y={sum}");
}

// function return values
const PI: f64=3.141592;
fn circle_area(radius: f64)->f64{   // ->반환될 값의 타입
    let r2=radius*radius;
    println!("entered radius: {radius}");
    PI*r2
}