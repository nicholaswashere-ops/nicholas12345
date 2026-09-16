use std::io; 
 fn main(){
    println!("I am Nicholas");
    println!("who are you");/* 
    placing*/
    println!("format {} an argument", "as");
    println!("I see you {} its been closer than this", "Nicholas");
// hello this is my first comment
/*you 
can always improvise*/
//variables
let salary = 25_000;
let fines:f64 = 25_000.00;
println!("Monthly salary is ${} and fines are ${}",salary,fines);
/*:i32 used for whole numbers
f64 for floats
cannot add f64 to i32
let variable_name: variable_type = value;*/
let score: i32 = 670;
let total = score+salary;
println!("Total:{}",total);
//A char is a single character data type that stores one character (letters, numbers, symbols, etc.).
let _nice : char= 'b';
let _nice : char= '%';
/*A char is a single character (e.g., 1, 6, %, b, p, ., T).
A String consists of multiple chars. Initialize strings using double quotation marks:*/
let _game: String="That was nice".to_string();

//A boolean type has only 2 possible values: true or false.
let _boy :bool = true;
let _girl:bool = false;

// murable variables
let mut age:i32 = 13;
println!("HIS AGE IS  {} ",age);
age = 56;
println!("He is now {}yrs old",age);
/*the variable can be changed without prducing an error
if also adding variable_types it should only be done once on the muted variable*/
//Guessing Variables
println!("choose either Mark ,Gary ,Jose");
let mut name =String::new();
io::stdin()
   .read_line(&mut name)
   .expect("failed to read name");
println!("you selected the name {} ", name);
println!("Thank you {}",name);
// NB \n means move to the next line

// Integer to Float conversion
let number: i32 = 5;
let decimal: f64 = number as f64;
// becomes 5.0
//typecasting 
// Float to Integer conversion
let decimal: f64 = 9.7;
let number: i32 = decimal as i32;
// becomes 9 (decimal part is truncated)

//The modulo operator % gives the remainder of a division:
let result: f64 = 5.2 % 2.0; // result is 1.2
let result: f64 = 7.8 % 3.5; // result is 0.8

/* to insert it directly into your line of code std::io::Stdin
to gain remainders of a division use % as the calculation symbol in its performedd variable*/
let mut a: i32 = 5;
a +=  3; // a holds 8
}

