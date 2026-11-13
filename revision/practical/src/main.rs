use std::io::stdin;

/*fn main() {
    let p = input("What is your name?");
    let s = input("What is your age?");
    println!("{}", p);
    println!("{}",s);
    println!("Hello, world!");
}

fn  input(que: &str) ->  String {
    println!("{}", que);
    let mut input = String::new(); 
    stdin().read_line(&mut input).expect("Failed to read input");
    input
}*/
//Anything inside a{} bracket is called a block                                                                    
fn main (){
    println!("Area formula");

   let c = square(7);
println!("area: {}",c);
    
}
//pow(2)power of 2 for undefined
//powf()power of 2 for f64,flot

fn square(l:u8)->u8{
    println!("Input your length");

   let area = l*l;
        
 return area
    

}