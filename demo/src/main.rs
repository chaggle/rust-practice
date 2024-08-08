// fn main() {
//     let x = 10;
//     println!("The value of x is: {}", x);
//     x = 6; // this is not allow in rust becase `let x` is no changeable
//            // if you want it changeable, it maybe `let mut x`
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let mut x = 10;
//     println!("The value of x is: {}", x);
//     x = 6; like this
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let _x = 1;
//     let y = 10; // help: if this is intentional, prefix it with an underscore: `_y`
// }

// fn main() {
//     let (a, mut b):(bool, bool) = (true, false);
//     // a = true,不可变; b = false，可变
//     println!("a = {:?}, b = {:?}", a, b);
//     b = true;
//     assert_eq!(a, b)
// }

// struct Struct{
//     e :i32
// }

// fn main() {
//     let (a, b, c, d, e);
//     (a, b) =  (1, 2);

//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct {e, ..} = Struct{e: 5};
//     assert_eq!([1, 2, 3, 4, 5], [a, b, c, d, e])
// }

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x is: {}", x);
//     }

//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let mut spaces = "   ";
//     spaces = spaces.len(); //显然，Rust 对类型的要求很严格，不允许将整数类型 usize 赋值给字符串类型。
//                            // usize 是一种 CPU 相关的整数类型，在数值类型中有详细介绍。
// }
