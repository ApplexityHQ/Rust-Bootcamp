// fn main() {
//     println!("Hello, world!");
// }


/*
===================================================
================= Variables (nums, strings, bools)
================= Conditionals
================= Loops
================== Functions
===================================================
*/



// Que: Check if a number is even.
// fn main () {
//     let ans = is_even(20);
//     println!("{}", ans);
// }
// // i  means signed (i32), and u means unsigned (u32)
// fn is_even (num: i64) -> bool {
//     if num%2 == 0 {
//         return true;
//     } else {
//         return false;
//     }
// }





// que: Write a function fib that finds the fibbonacci of a number it takes as input.
// fn main() {
//     println!("{}", fib(10));
// }

// fn fib (num: u32) -> u32 { 
//     let mut first = 0;
//     let mut second = 1;

//     if num == 0 {
//         return first;
//     }    

//     if num == 1 {
//         return second;
//     }

//     for _ in 0..(num - 2) {
//         let temp = second;
//         second = second + first;
//         first = temp;
//     }
//     return second;
// }


// Que: Write a function get_string_length that takes a string as an input returns its length.
// fn main() {
//     let name = String::from("my name is applexity");
//     let len = get_string_length(name);
//     println!("The length of the string is {}", len);
// }
// fn get_string_length(str: String) -> usize {
//     str.chars().count()
// }


// # Understanding Structs
struct User {
    first_name: String,
    last_name: String,
    age: i32,
}
fn main() {
    let user = User {
        first_name: String::from("Applexity"),
        last_name: String::from("Ox"),
        age: 20,
    };

    println!(" First Name : {} Last Name: {} Age: {}" , user.first_name, user.last_name, user.age);
}