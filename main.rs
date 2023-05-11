// Lesson 1
//////////////////////////////////////////////////
///  
// fn main() {
//     let massage = "Hello World";

//     let x: i32 = 42;
//     let pi: f64 = 3.14;
//     let is_rust_fun: bool = true;
//     let letter_a: char = 'a';

//     fn add(x: i32, y: i32) -> i32 {
//         y + x
//         // return  x + y; // also possible to discard the return keyword but not using ; is mandatory in this case
//     };

//     let x = 42;

//     if x >= 0 {
//         println!("x is not negative");
//     } else {
//         println!("x is negative ");
//     }

//     let mut i = 1;

//     while i <= 5 {
//         println!("{}", i);
//         i += 1
//     }
// }

//////////////////////////////////////////////////////////////
// Lesson 2

// fn main() {
//     // adding _ at the beginning of a variable name we avoid us from getting an error in execution from cargo
//     // Booleans
//     let _my_first_bool: bool = true;
//     let _my_second_bool: bool = false;

//     // in rust we need to manage memory space for integers, it means we can use less space then the maximum. but it means we are responsible for managing it .
//     // 8, 16, 32, 64, 128 bits (digits) integers are acceptable.
//     // Integers
//     let _days_of_week: i8 = 7;
//     // so infect it means that we can specify the amount of memory we need, using integer 7 only requires 8 bits of memory. (notice that integer 0 also takes a bit so for i8 (8bit integer) 7 is the maximum acceptable)

//     let _number_of_users: i64 = 120000;

//     let _number_of_tokens: u128 = 10000;

//     let _just_a_number = 0; // if the integer type is not specified it will compile automatically to i32

//     // if we dont want to go with this approach and not handle bits its possible not to specify the type and use i32 which is compatible for most cases.

//     // Floating Point Number
//     // 32, 64 bits
//     let pi: i32 = 3.14;

//     // Characters
//     let my_char: char = '1';

//     // Strings
//    let _message: &str = "Hi, Itay";

//    let _my_string = $String::from("Hi, Itay")

//    // Arrays
// //    declaring
//    let _days_of_week: [&str; 7] = [
//     "Sunday",
//     "Monday",
//     "Tuesday",
//     "Wednesday",
//     "Thursday",
//     "Friday",
//     "Saturday",
//    ];

//    // getting elements from array
//    let _first_element = _days_of_week[0];
//    let _last_day_of_week = _days_of_week[_days_of_week.len() - 1];

//     // Slices
//     let slice = &_days_of_week[1..3];
//     let _first_element_of_slice = slice[0];

//     // Tuples
//     let person = ("Alice", 30)

//     let name = person.0;
//     let age = person.1;

//     // Unit Type
//     let _unit_type = ();

//     // Variables
//     let mut num = 5;
//     // in rust the variables are immutable, if we need to mutate it we need to declare it with the mut keyword.
//     num = 6;

// }

//////////////////////////////////////////////////////////////
// Lesson 3

// fn main() {
//     let sum = add(3, 5);
//     println!("The sum is {}", sum);

//     let _day_of_week = "Sunday";

//     if _day_of_week == "Sunday" {
//         println!("The race day!")
//     } else if _day_of_week == "Saturday" {
//         println!("Qualifing today!")
//     } else {
//         println!("Patiently wiat for the race day")
//     }

//     // while loop
//     let mut counter = 0;

//     while counter <= 5 {
//         println!("Counter value is {}", counter);
//         counter += 1;
//     }

//     // for loop
//     let numbers: [i32; 5] = [1, 2, 3, 4, 5];
//     for number in numbers {
//         println!("Number is {}", number)
//     }
//     // for number in 1..=5 {
//     // println!("Number is {}", number)
//     // }

//     // loop
//     counter = 0;

//     loop {
//         println!("Counter2 value is {}", counter);
//         counter += 1;

//         if counter == 6 {
//             break;
//         } 
//     }

//     // 
//     let num = 5;

//     match num {
//         1 => {
//             println!("The number is one");
//             println!("This is the first match arm");
//         },
//         2 => println!("The number is two"),
//         3 => println!("The number is three"),
//         _ => println!("The number is something else"), // this is like the else for the cases that are not met
//     }

//     let result = match num {
//         1 => "The number is one",
//         2 => "The number is two",
//         3 => "The number is three",
//         _ => "The number is something else",
//     };

//     println!("The result is {}", result)



// }

// // Functions
// fn add(x: i32, y: i32) -> i32 {
//     let result = x + y;
//     return result;
// }

// fn no_param() -> i32 {
//     println!("This just works");
//     1
// }


/////////////////////////////////////////////////////////////
// Lesson 4

// fn main() {
//     let _s1 = String::from("Hello");

//     let _s2 = _s1;
//     // println!("Value of s1 is {}", s1);

//     let x: i32 = 5;

//     let y = String::from("Patika");

//     let z = y;

//     println!("Value of x is {}, Value of z is {} , Value of y is {}", x, z, y);
// };


// fn main() {
//     fn mine() {
//         let _my_string = String::from("Hello, World!");
//         let slice = &_my_string[0..5];
//         println!("{}", slice);
//         println!("{}", _my_string);
//     }
//     mine();
// }


/////////////////////////////////////////////
//////// Lesson 5 \\\\\\\
// Borrowing and References using & operator

// From video:

// &: Immutable referencing (its impossible to mutate the origin variable).
// fn main() {
//     fn print_string(s: &String) {
//         println!("{}", s);
//     }
    
//     let my_string = String::from("Hello World!");
//     print_string(&my_string);  
// }


// &mut: Mutable referencing (its possible to mutate the origin variable).
// fn main() {
//     let my_string = String::from("Hello, World!");

//     let my_ref = &my_string;

//     // println!("My reference is {}", my_ref)

//     let my_string = String::from("Hello, World!");
//     // print_string(&my_string);

//     // println!("I still got my string {}", my_string) // by referencing the variable (_my_string) is still accessible even though we used it in print_string(). if it wasent just a reference my_string would have been delited from the stack at this point in time which is after my_string have been diclared and used. 
    
//     let mut my_string = String::from("hello");

//     change_string(&mut my_string);

//     println!("{}", my_string);

//     let first_immutable_reference =  &my_string;
//     let second_immutable_reference =  &my_string;

//     // println!("First immutable reference value {}", first_immutable_reference);
//     // println!("Second immutable reference value {}", second_immutable_reference);

//     let first_mutable_reference = &mut my_string;
//     println!("First mutable reference value {}", first_mutable_reference); 

//     // println!("First immutable reference {}", first_immutable_reference) // this line produces an error. after we assigned the first_mutable_reference variable to be a reference to my_string in a mutable way using &mut keyword. at this point in program time the mutable variable &mut my_string will be used and discarded after use from the stack and wont be available to the printIn! method.

//     let second_mutable_reference = &mut my_string;

//     println!("{}", second_mutable_reference);
//     // println!("First mutable reference value {}", first_mutable_reference); // this throws an error. the same mutable variable they both refer to has passed the ownership from the first owner first_mutable_reference to the second owner second_mutable_reference. !!! in other words you can only have one mutable reference at a time !!!


//     // Dangling reference
//     // a reference that points to a memory location that has been deallocated. 

//     let new_string = String::from("new string");
//     let new_string_reference = return_reference(&new_string);

//     // println!("new string {}", new_string)

//     let newer_string = new_string;

//     println!("new string reference {}", new_string_reference)

//     // from lesson:



    
// }



// fn print_string(s: &String) {
//     println!("{}", s);
// }

// fn change_string(s: &mut String) {
//     s.push_str("world");
// }

// fn return_reference(some_string: &String) -> &String {
//     some_string
// }



// // let mut my_string = string::from("hello");


/////// TASK 1 \\\\\\\
// Implement a basic program that uses ownership concepts

fn main() {

    let _s1 = String::from("Hello");
    let _s2 = String::from(" Itay (-:");

    let message  = concat_strings(&_s1, &_s2);
    println!("{}", message);

}

fn concat_strings(s1: &String, s2: &String) -> String {

    let mut concated = s1.clone();
    concated.push_str(s2);
    concated
    
}

