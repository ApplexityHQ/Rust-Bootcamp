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

// struct User {
//     first_name: String,
//     last_name: String,
//     age: i32,
// }
// fn main() {
//     let user = User {
//         first_name: String::from("Applexity"),
//         last_name: String::from("Ox"),
//         age: 20,
//     };

//     println!(" First Name : {} Last Name: {} Age: {}" , user.first_name, user.last_name, user.age);
// }

// Implementing a struct

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect = Rect {
//         width: 30,
//         height: 50,
//     };
//     println!("The area of the rectangle is {}", rect.area());
// }

// Structs lets you structure data together.

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("Applexity"),
//         email: String::from("applexity@gmail.com"),
//         sign_in_count: 1,
//     };
//     print!("User 1 username: {:?}", user1.username);
//     print!("User 1 active: {:?}", user1.active);
//     print!("User 1 email: {:?}", user1.email);
//     print!("User 1 sign_in_count: {:?}", user1.sign_in_count);
// }

// Enums let you enumerate over various types of a value.

// enum Direction {
//     North,
//     East,
//     West,
//     South,
// }

// fn main() {
//     let my_direction = Direction::North;
//     let new_direction = my_direction;
//     move_around(new_direction);
// }

// fn new_direction(direction: Direction) {
//     // implement logic to move a character around
// }

// Option Enum (For Null Values in Rust)

// fn find_first_a(s: String) -> Option <i32> {
//     for (index, character) in s.chars().enumerate() {
//         if character === 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

// fn main() {
//     let my_string = String::from("applexity");
//     match find_first_a(my_string) {
//             Some(index) => println!("The letter 'a' is found at index: {}", index);
//             None => print!("The letter 'a' is not found in the string.");
//     }
// }

// Results
// Write a funtion that reads the contents of a file.

// use std::fs;

// fn main() {
//     let greeting_file_result = fs::read_to_string("a.txt"); // Result

//     match greeting_file_result {
//         OK(file_content) => {
//             println!("File read successfully: {:?}", file_content);
//         },
//         Err(error) => {
//             println!("Failed to read file: {:?}", error);
//         }
//     }
// }

// Trait Implementation:

// trait _Greet {
//     fn say_hello(&self);
// }

// struct _Person {
//     name: String,
// }

// // Implement the Greet trait for the Person struct
// impl _Greet for _Person {
//     fn say_hello(&self) {
//         println!("Hello, my name is {}", self.name);
//     }
// }

// Matching on enums

/*

fn mina() {
    println("hi");
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Variant can hold data
}

enum UsState {
    Alabama,
    Alaska,
    // ... other states
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // Bind the inner value to a variable 'state'
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

*/

/*
====================
Package management
===================
*/

//

// External packages/crate
// > cargo init <crate_name>

// creading current time.

// use chrono::Local;
// fn main() {
//     let now = Local::now();
//     println!("Current Time is {}", now);
// }

/*
====================
Memory management
===================
*/

// Ex 1: Simple example of memory for a simple program.
// fn main() {
//     let a = 1; // stored in stack
//     let b = 2; // stored in stack
//     let sum = a + b; // stored in another stack frame
//     println!("Sum if {}", sum);
// }

// fn find_sum(a: i32, b: i32) -> i32 {
//     let ans = a + b;
//     return ans;
// }

// Ex 2: How it's stored in heap (allocated at runtime) & its addresss is saved in stack frame

// fn main() {
//     let name = String::from("Hello");
//     println!("Name is {}", name)

//     let mut name2 = String::from("Hello2 ");
//     name2.push_str("World");
// }

// stored in stack:
/*
        stored on the stack:
            - numbers, booleans, fixed sized arrays, structs, refrences
        stored on the heap:
            - strings, vectors, hashmap, large arrays/structs that can't fit in the stack
*/

/*
===========================
jargons to be covered: (Already Covered in Previous Cohort repo. just revise here)
1. Ownership
2. Moving
3. Borrowing
4. References
===========================
*/

/*  1. Ownership
        - Rust does not have garbage collector, but it has a concept of ownership which helps in romving data from heap as soon as the owner goes out of scope from stack.

    2. Move
        - owernership is moved once it gets transfered from one to another.
        - and old one becomes a null or dangling pointer (so it becomes invalid)
        - Insted of pointing 2 owners, USE: .clone()
    3. Borrowing
        - Borrowing = temporarily using data without owning it
        - &T      // immutable borrow (read-only)
        - &mut T  // mutable borrow (read & write)



*/

// ex: Moving example (the below code won't compile)
// fn main() {
//     let s1 = String::from(Hello);
//     let s2 = s1;
//     println!("Number is {}", s2);
// }

// Ex 2: Moving example 2
// fn create_string() {
//     let s1 = String::from("hey");
//     let s2 = s1;

//     //print the string.
//     print!("{}", s1);
// }

// fn main() {
//     //call the function
//     create_string();
// }

// Ex 3:

// fn main() {
//     let mut s1 = String::from("aplexity");
// }

// fn do_something(s2:String) {
//     print!("{}", s2);
//     return s2;
// }

// Ex 4: Borrowing example

// fn create_string() {
//     let s1 = String::from("hello");
//     print_str(&s1); // borrowed
// }

// fn print_str(s2: &String) {
//     println!("{}", s2);
// }

// fn main() {
//     // call the function
//     create_string();
// }

/*
===========================
Advanced Rust (slightly)
===========================
1. Collections, vectors
2. Iterators
3. Hashmaps
4. Strings, &str and slices
5. Generics
6. Traits
7. Multithreading
8. Macros
9. Futures
10. Async/await and tokio
11. Lifetimes.
===========================


*/

// 1. Vector

// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);

//     println!("{:?}", vec);

// }

// Ex on vec: Write a function that takes vector as an input and returns a vector with even values.

// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3); // [1,2,3]
//     println!("{:?}", is_even(&vec))  ;

// }

// fn is_even(vec: &Vec<i32>) -> Vec<i32> {
//     let mut new_vec = Vec::new();
//     for val in vec {
//         if val % 2 == 0 {
//             new_vec.push(*val); // dereference to the variables. using (*)
//         }
//     }
//     return new_vec;
// }

// Approach 2:
// fn even_values(v: &mut Vec<i32>) {
//     let mut i = 0;
//     while i < v.len() {
//         if v[i] % 2 !=0 {
//             v.remove(i);
//         } else {
//             i += 1;
//         }
//     }
// }

// fn main() {
//     let mut vec = Vec::new(); // vec![1,2,3,4];
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
//     even_values(&mut vec);
//     print!("Updated vector is {:?}", vec);
// }

// Explicitly giving typed using generics Vec<T>

// fn main() {
//     let numbers = vec![1,2,3];
//     for numbers in numbers {
//         println!("{}", numbers);
//     }
// }

// explicitly giving vecters a type using generics

// fn main() {
//     let numbers: Vec<i32> = vec![1,2,3];
//     for number in numbers {
//         println!("{}", number);
//     }
// }

// 2. Hashmaps

/*
    - Hashmaps stores key value pair in rust
    - Similar to objects in Js
    - Dict in Python
    - HashMaps in Java
*/

// use std::collections::HashMap;

// fn main() {
//     let mut users = HashMap::new();

//     users.insert(String::from("Applexity"), 22);
//     users.insert(String::from("Applexity 2"), 32);

//     // {
//     //      applexity: 22,
//     //      applexity2:23
//     // }

//     let first_user_age = users.get("Applexiiiityy");

//     match first_user_age {
//         Some(age) => println!("Age is {}", age),
//         None => println!("User not found in the db"),
//     }

// }

// 3. Iterators

// ex:
// let v1 = vec![1,2,3];
// let v1_iter = v1.iter();

// ex: iterator is something like
// fn main() {
//     let v1 = vec![1,2,4];
//     for val in v1 {
//         print!("Got it: {val}");
//     }
// }

// // ex: iterator using loops
// fn main() {
//     let nums = vec![1,2,4];

//     for val in nums {
//         println!("{}", val);
//     }

// }

// ex: iterating after creating an 'iterator'
// fn main() {
//     let nums = vec![1,2,4,5];
//     let iter = nums.iter();

//     for value in iter {
//         println!("{}", value);
//     }
// }

// fn main() {
//     let mut nums = vec![1,3,4];
//     let iter = nums.iter_mut();

//     for value in iter {
//         *value = *value + 1;
//     }

//     println!("{:?}", nums);
// }

// fn main() {
//     let mut v1 = vec![1, 3, 4];

//     let v1_iter = v1.iter();

//     for val in v1_iter {
//         println!("{}", val);

//         println!("{:?}", v1);
//     }
// }

// Types of Iterators
// 1. .iter
// 2. .iter_mut
// 3. into_iter()

// Iterators (Consuming Adaptors)

// fn main () {
//     let v1 = vec![1,3,4];
//     let v1_iter = v1.iter();

//     let sum = v1_iter.sum();

//     println!("sum is {}", sum);

//     for i in v1_iter {}
//     println!("{:?}", v1);
// }

// filter even values
// fn main() {
//     let v1 = vec![1,3,4,5];
//     let iter = v1.iter();

//     let iter2 = iter.filter(|x| *x % 2 == 0);

//     for x in iter2 {
//         println!("{}", x);
//     }
// }

// filter odd values
// fn main() {
//     let v1 = vec![1,3,4,5];
//     let iter = v1.iter();

//     let iter2 = iter.filter(|x| *x % 2 == 1);

//     for x in iter2 {
//         println!("{}", x);
//     }
// }

// Q. Write the logic to first filter all odd values then double each value and create a new vector

// fn filter_and_map(v) -> Vec<i32> {
//     let new_iter = v.iter().filter(|x| *x % 2 == 1).map(|x| x + 1);
//     let new_vec = new_iter.collect();
//     return new_vec;
// }

// fn main() {
//     let v1 = vec![1,2,4,5];
//     let ans = filter_and_map(v1);
//     println!("{:?}", ans);
// }










// 4. Strings & Slices

// creating a string, mutating a string, deleting a string.
// fn main() {
//     let mut name = String::from("Applexity");
//     name.push_str(" Ox");
//     println!("name is {}", name);
//     name.replace_range(4..name.len(), "");
//     println!("NAME is {}", name);
// }
// output: Appl



// Q: Write a function that takes a string as an input and returns the first word from it.

// Approach 1

// problem with above code ? 
// -- We take up double the memory -- if the 'name' string getss 'cleaed', 'ans' still has 'hello' as the value in it.
// we want to 'view' in to the original string and not copy it over.


// Approach 2:

// fn main() {
//     let mut word = String::from("Hello World");

//     println!("{}", word2);
// }

// fn find_first_word(word: String) -> &str {
//     let index = 0;
//     for i in word.chars().enumerate() {
//         if i == ' ' {
//             break;
//         } 
//         index = index + 1;
//     }
//     return &word[0..index];
// }


// fn main() {
//     let arr = [1,2,4,5,4];
//     let arr_slice = &arr[0..1];
// }









// 5. Generics


// ex of code1

// fn main() {
//     let bigger = largest_i32(1,2);
//     let bigger_char = largest('a', 'b');
//     println!("{}", bigger);
//     println!("{}", bigger_char);
// }

// fn largest_i32(a: i32, b: i32) -> i32 {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }

// fn largest_char(a: char, b: char) -> char {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }



// rust quivalent code of the above code1 , 

// fn main() {
//     let bigger = largest(1,2);
//     let bigger_char = largest('a', 'b');
//     println!("{}", bigger);
//     print!("{}", bigger_char);
// }
// // generic
// fn largest<T: std::cmp::PartialOrd>(a:T , b:T) -> T {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }









// 6. Traits

// similar to abstract classes of Java
// similar to interfaces of Js

pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User {} is {} years old", self.name , self.age);
    }
}

struct Fix;
impl Summary for Fix {}
impl Summary for User  {}


fn main() {
    let user = User {
        name: String::from("Applexity");
        age: 21;
    }
    println!("{}", user.summarize());
}

fn notify(u: impl Summary) {
    println!("{}", u.summarize()) 
} 

