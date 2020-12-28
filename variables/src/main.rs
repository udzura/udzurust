#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let x = 5;
    // println!("The value of x is: {}", x);
    // let guess: f32 = "4.2".parse().expect("Not a number!");
    // println!("The value of x is: {}", guess);

    // let a = [10, 20, 30, 40, 50];

    // for element in a.iter() {
    //     // 値は{}です
    //     println!("the value is: {}", element);
    // }
    // println!("Range: {:?}", 1..4);

    // ownership_check();
    // ownership_check2();
    // ownership_check3();

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);
}

fn ownership_check() {
    let s1 = String::from("hello");
    let s2 = s1;
    let s1 = s2;

    println!("{}, world!", s1);
}

fn ownership_check2() {
    let s1 = "hello";
    let s2 = s1;

    println!("{}, {}, world!", s1, s2);
}

fn ownership_check3() {
    let s1 = String::from("hello");

    {
        let _p1 = & s1;
    }

    {
        let p2 = &s1[2..3];

        println!("{:?}, world!", p2);
    }
}
