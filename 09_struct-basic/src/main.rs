// 구조체 선언
struct User{
    name: String,
    email: String,
    active: bool
}

// utility function
fn build_user(name: String, email: String)->User{
    User{
        name: name,
        email: email,
        active: true
    }
}

// tuple struct (튜플 구조체)
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // 구조체 생성
    let mut user1=User{
        name: String::from("user1"),
        email: String::from("user@example.com"),
        active: true
    };

    println!("user1 name: {}", user1.name);
    
    user1.email=String::from("user1@example.com");
    println!("user1 email: {}", user1.email);

    // 함수를 활용한 struct 생성
    let mut user2=build_user(String::from("user2"), String::from("user2@example.com"));

    // 이미 존재하는 구조체의 인스턴스로부터 새 구조체 생성 (1)
    let user3=User{
        name: user1.name,   // user1의 프로퍼티를 활용했기 때문에 user1 name의 소유권을 빼앗음
        email: user1.email, // user1의 프로퍼티를 활용했기 때문에 user1 email의 소유권을 빼앗음
        active: false
    };
    println!("user3 email: {}", user3.email);
    // println!("user1 email: {}", user1.email);    // error (소유권이 없음) .clone() 써서 해결

    // 이미 존재하는 구조체의 인스턴스로부터 새 구조체 생성 (2)
    let user4=User{
        active: false,
        ..user3
    };

    // tuple struct (일반적인 tuple보다 더욱 명시적으로 사용 가능, 타입을 엄격히 관리 가능)
    let color=Color(1, 2, 3);
    let point=Point(1, 2, 3);
}
