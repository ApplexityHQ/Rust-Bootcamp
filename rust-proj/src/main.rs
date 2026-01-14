fn main() {
    // println!("Hello, world!");

/* 
#1 ========== Simple variables in Rust ========= 
*/
    // ==== 1. Numbers ====

    // signed and unsigned. i(signed) , u(unsigned) 
    // let x: i32 = 1;
    // println!("{}", x);

    // let mut a = 35;
    // a = a + 2;
    // println!("{}", a);
    

    // let y = 1000;
    // let z = 100.309;
    // println!("y: {}", y);
    // println!("z: {}", z);


    // ==== 2. Numbers ====

    // let is_male = false;
    // let is_above_18 = true;
    
    // if is_male {
    //     println!("You are a male");

    // } else {
    //     println!("You are not a male");
    // }

    // if is_male && is_above_18 {
    //     print!("You are a legal male");
    // }



    // ==== 3. Strings ====
    // let greeting = String::from("hello world");
    // println!("{}", greeting);

    // let char1 = greeting.chars().nth(1);
    // print!("char1: {}", char1.unwrap());





/* 
#2 ========== Conditionals, Loops ========= 
*/

    // ==== 2.1 If Else Loop ====
    // let is_even = true;

    // if is_even {
    //     print!("The number is even")
    // } else if !is_even {
    //     print!("The number is Odd")
    // }


    // ==== 2.2 For Loop ====
    // for i in 0..10 {
    //     print!("{}", i);
    // }


    // ==== Q: Print the first name of sentence ====
    // let sentence = String::from ("Jhon Doe");
    // let first_word = get_first_word(sentence);

    // let n = 10;
    // for i in 0..n {
    //     println!("Hello World {}", i);
    // }

    // print!("First word is: {}", first_word);

    // fn get_first_word(sentence :String) -> String {
    //     let mut ans = String::from("");
    //     for char in sentence.chars() {
    //         ans.push_str(char.to_string().as_str());
    //         if char == ' ' {
    //             break;
    //         }
    //     }
    //     return ans;
    // }
    

    
    // NOTE:
        // put an underscore upfront (_) when unused error comes




/* 
#3 ========== Functions ========= 
*/
    // ==== 3.1 Functions definition ====
    // fn _do_sum(a: i32, b: i32) -> i32 { 
	// return a + b;
    // }





/* 
#4 ========== Mutability ========= 
*/    
    // By default, variables are mutable by default.

    // let x = String::from("hii there"); // => Immutable (cannot be changed)
    // x.push_str("asd");
    // // error x is immutbale
    // println!("{}", x);

    








/* 
#4 ========== Stack v/s Heap ========= 
*/
    // ==== Memory in action ====

  
        // stack_fn();   // Call the function that uses stack memory
        // heap_fn();    // Call the function that uses heap memory
        // update_string();  // Call the function that changes size of variable at runtime
    


}















































    // fn stack_fn() {
    //     // Declare a few integers on the stack
    //     let a = 10;
    //     let b = 20;
    //     let c = a + b;
    //     println!("Stack function: The sum of {} and {} is {}", a, b, c);
    // }

    // fn heap_fn() {
    //     // Create a string, which is allocated on the heap
    //     let s1 = String::from("Hello");
    //     let s2 = String::from("World");
    //     let combined = format!("{} {}", s1, s2);
    //     println!("Heap function: Combined string is '{}'", combined);
    // }

    // fn update_string() {
    //     // Start with a base string on the heap
    //     let mut s = String::from("Initial string");
    //     println!("Before update: {}", s);

    //     // Append some text to the string
    //     s.push_str(" and some additional text");
    //     println!("After update: {}", s);
    // }