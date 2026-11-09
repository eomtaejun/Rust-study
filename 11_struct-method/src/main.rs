#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

/*
    - rectangle: &rectangle
    - &self: self: &self (축약 표현)
    - 소유권 가져갈 수도 있고 (불변/가변) 임대해도 됨
*/
impl Rectangle{
    fn area(&self)->u32{    // 기본형: self: &Self or self: &mut Self
        self.width*self.height
    }
}

// associated 연관 함수
impl Rectangle{
    fn square(size: u32)->Rectangle{
        Rectangle{
            width: size,
            height: size
        }
    }
}

fn main() {
    /*
        - function과 비슷, parameter를 받고 return값을 준다
        - struct, enum, trait 문맥 안에 정의
     */

    let rect=Rectangle{
        width: 20,
        height: 30
    };

    println!("area size is {}", rect.area());
    println!("square: {:?}", Rectangle::square(20));
}