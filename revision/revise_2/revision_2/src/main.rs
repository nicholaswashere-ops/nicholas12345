use std::io;
//a looop mechanism for ci and amt that ends with n
fn main() {
    loop{
    let mut input_1 =String::new();
    let mut input_2 =String::new();
    let mut input_3 =String::new();
    println!("insert the principal");

     io::stdin().read_line(&mut input_1).expect("This is the future");
     let p :f64=input_1.trim().parse().expect("This is the past");
        
    println!("insert the rate");

     io::stdin().read_line(&mut input_2).expect("This is not a String");
     let r:f64= input_2.trim().parse().expect("This is not an integer");
      
    println!("insert the time");

     io::stdin().read_line(&mut input_3).expect("This is not a String");
     let t:f64 =input_3.trim().parse().expect("this is measured in seconds");
      
     let a = p*(1.0+ r/100.0).powf(t);
           
     let ci = a-p;
     println!("PRINCIPAL: ${}.00",p);
     println!("RATE: {}% ",r);
     println!("TIME: {}s",t);
     println!("AMOUNT: ${:.2}",a);
     println!("COMPOUND INTEREST: ${}",ci);
  
     println!("Enter another calculation? (y/n)");

        let mut code =String::new();
        let _ =io::stdin().read_line(&mut code);

if code.trim().to_lowercase() == "n"{
    break;
}



}

println!("program ended");


}
