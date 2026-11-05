fn main() {
    // 조건문
    let x=4;
    if x%3==0{
        println!("The remainder when x is divided by 3 is 0");
    } else if x%3==1{
        println!("The remainder when x is divided by 3 is 1");
    } else{
        println!("The remainder when x is divided by 3 is 2");
    }

    let condition=true;
    let y=if condition{3} else{5};
    println!("y: {y}");

    // 반복문
    // loop
    let mut cnt=0;
    let x=loop{
        println!("loop {cnt}");
        cnt+=1;

        if cnt>2{break cnt;}
    };
    println!("x: {x}");

    // while
    let arr=[1, 2, 3, 4, 5];
    let mut idx=0;
    while idx<arr.len(){
        println!("arr[{}]: {}", idx, arr[idx]);
        idx+=1;
    }

    // for
    for value in arr{
        println!("value: {value}");
    }

    for i in (0..5).rev(){
        println!("for {i}");
    }
}