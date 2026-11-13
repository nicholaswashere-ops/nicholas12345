//Note; that the name of the crab for rust is ferris.



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
let _salary = 25_000;
let _fines:f64 = 25_000.00;
println!("Monthly salary is ${} and fines are ${}",_salary,_fines);
/*:i32 used for whole numbers
f64 for floats
cannot add f64 to i32
let variable_name: variable_type = value;*/
let _score: i32 = 670;
let _total = _score+_salary;
println!("Total:{}",_total);
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
let mut _age:i32 = 13;
println!("HIS AGE IS  {} ",_age);
_age = 56;
println!("He is now {}yrs old",_age);
/*the variable can be changed without prducing an error
if also adding variable_types it should only be done once on the muted variable*/
//Guessing Variables
println!("choose either Mark ,Gary ,Jose");
let mut _name =String::new();
io::stdin()
   .read_line(&mut _name)
   .expect("failed to read name");
println!("you selected the name {} ", _name);
println!("Thank you {}",_name);
// NB \n means move to the next line

    let a = 5;
    let p= &a;//To store the memory address of a 
    println!("{:p}",p);//to print the memory address of a
    println!("{:p}",&p);//To print the memory adderess of p
    let mut x =1;
    
    {  let mut x=3;//Its counted and pass by lines by arranged
        let  p =&mut x;
      
        *p=2;
        println!("{}",x);//Sending 2 into p into x without making it a copy ; pass by reference
    }//Shadowing
println!("{}",x);

//&- memory address, * pointer
//Search Stack overflow

/*HOw to work with an AI agent
/The cause of the problem;find 
/explain the problem;understand
/can you fix it;fix*/

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


// Integer to Float conversion
let _number: i32 = 5;
let _decimal: f64 = _number as f64;
// becomes 5.0
//typecasting 
// Float to Integer conversion
let _decimal: f64 = 9.7;
let _number: i32 = _decimal as i32;
// becomes 9 (decimal part is truncated)

//The modulo operator % gives the remainder of a division:
let _result: f64 = 5.2 % 2.0; // result is 1.2
let _result: f64 = 7.8 % 3.5; // result is 0.8

/* to insert it directly into your line of code std::io::Stdin
to gain remainders of a division use % as the calculation symbol in its performedd variable*/
let mut _a: i32 = 5;
_a +=  3; // a holds 8

//   means not equal to !=
/*operators /meaning / example.
==  Equal 1 == 2 returns false
!= Not Equal   1 != 2 returns true
>  Greater Than   1 > 2 returns false
<  Lower Than  1 < 2 returns true
>= Greater or Equal  1 >= 2 returns false
<= Lower or Equal 1 <= 2 returns true*/
let _var1: i32 = 13;
let _var2: i32 = 12;
let _var3: bool = _var1 != _var2; // var3 will hold true

// different ways of outputting a sttring

let _string1: String = "hello".to_string();
let _string2: String = String::from("hello");
let _string3 = "hello".to_owned();
 println!("{}",_string3);

/*NB:  a crate is a collection of Rust source code files.
using cargo in  cmd
We can create a project using cargo new.
We can build a project using cargo build.
We can build and run a project in one step using cargo run.
We can build a project without producing a binary to check for errors using cargo check.
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.*/
let b1: bool = (5 > 3) && (1 == 1); // true (both operands are true)
let b2: bool = !(5 == 4) || (5 == 2); // true (first operand is true)
let b3: bool = !(1 == 1) || false; // false (both operands are false)
let b4: bool = !(3 > 4); // true (operand is false, so not operation returns true)
let b5: bool = !(5 > 10 || 5 > 1); // false (expression inside is true, so not operation returns false)

/* Logical operators are used to check combinations of comparisons that return true or false.

Operator Meaning  Example
&& And - true if all operands are true a && b
|| Or - true if any operand is true a || b
!  Not - true if the operand is false  !a, it just shows the opposite
Examples
:They work like logic gates
Logical Operators Part 2



Truth tables show the results of logical operator combinations:

AND (&&) operator: Returns true only when both operands are true

a  b  a && b
false false false
false true  false
true  false false
true  true  true
OR (||) operator: Returns true when either operand is true

a  b  a || b
false false false
false true  true
true  false true
true  true  true
NOT (!) operator: Reverses the boolean value

a  !a
false true
true  false

*/
/*selection iteraation and decision
selectiion lines of code
 iteratiion what the program
decision for ,while ,if ,else.
Types of a loop Normal loop ,
A block is a section of start to the end e.g from if to else
Scope means the level of visibility of a specific entity 
something on a higher scope can be execute a lower scope
Things of a lower scope can't be used in what is in the higher scope
if to else higher scope
if insdide the if to else is a lower scc
//(rustup doc --book) to open the rust book offline
*/



const NUMBER:u32 = 65;
//its just like let but with uppercase and not snake case
//it can work for both integers and strings and Rust’s naming convention for constants is to use all uppercase with underscores between words.
/*First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable. You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated. We’ll cover types and type annotations in the next section, “Data Types”, so don’t worry about the details right now. Just know that you must always annotate the type.

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.*/


//Shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{}heart_eyed_cat",heart_eyed_cat);

//Rust’s char type is 4 bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emojis; and zero-width spaces are all valid char values in Rust.
 let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

//this is how tuples are called
//Numbers isize and usize automatically fit in a digit to its form of whatever input it gives
let mut y: (i32, i32) = (1, 2);
    y.0 = 0;
    y.1 += 5;
//tuples can also be modified




/*scalar datatypes consist of :bool,char,integer,float
compound datatypes caonsist of :Strings, Tuples,Vectors and arrays
Arrays are useful when you want your data allocated on the stack, the same as the other types we have seen so far, rather than the heap (we will discuss the stack and the heap more in Chapter 4) or when you want to ensure that you always have a fixed number of elements. An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size because its contents live on the heap. If you’re unsure whether to use an array or a vector, chances are you should use a vector. Chapter 8 discusses vectors in more detail.

However, arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:*/


let a = [3; 5];
//it will be the same as :let a = [3,3,3,3,3]
//The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.

let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
//they can be accessed this way,unlike tuples.0


//calling a function
 another_function(5);


fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
//x:i32 is a parameter
print_labeled_measurement(5, 'h');


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
//it can also access multiple parameters
let y = {let x = 3;
   x + 1};

    println!("The value of y is: {y}");

//  Expressions do not include ending semicolons.  

//{}is an experession  while a statement iss jrusst the variable declaration : let h =78; under sttand the difference



    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    
}
//A whike loop runs a function like a let and can break automatically after the enfdd
/*A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop. It’s possible to implement behavior like this using a combination of loop, if, else, and break; you could try that now in a program, if you’d like. However, this pattern is so common that Rust has a built-in language construct for it, called a while loop. In Listing 3-3, we use while to loop the program three times, counting down each time, and then, after the loop, to print a message and exit.*/

    for number in (1..4).rev() {
        println!("{number}!");
    
    println!("LIFTOFF!!!");
}//for loops are limited
let day = 3;
let day_name = match day {
    1 => "Monday",
    2 => "Tuesday",
    3 => "Wednesday",
    _ => "Invalid day",
};
//understandinbg the match statement
//Commas separate match arms, not patterns within an arm. Use | to combine patterns.
//The underscore is functional code, not a placeholder. It actively matches remaining values.
//Match expressions must be exhaustive: the compiler verifies that all possible values are covered, either explicitly or with a default _ arm.
let day = 6;
let day_type = match day {
    1 | 2 | 3 | 4 | 5 => "Weekday",
    6 | 7 => "Weekend",
    _ => "Invalid",
};
//.rev()is to print backwards
 for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");



  let a :f32=4.0;
    let b :f32=a.powf(2.0);
    println!("a:{}",b);
    //For printing the Rate tot yhe 4^2 power of floats
   let a :i32=4;
    let b =a.pow(2);
    println!("a:{}",b);
    //For printing thethe rate to the power in integers

       let a :i32=4;
    let b =a.isqrt();
    println!("a:{}",b); 
    //For square rooting integers
       let a =4.0;
    let b =a.sqrt();
    println!("a:{}",b);
    //For Square rootinng floats
    
































}