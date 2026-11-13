struct Tasking{
    name:String,
    age:u8,
    occupation:String,
    date_created :String,
    task:String,
    date_exhausted:String

}
fn main() {
    let task1 =Tasking{
        name:"Richard".to_string(),
        age:32,
        occupation:"Lawyer".to_string(),
        date_created:"23rd October, 2024".to_string(),
        task:"To gain a degree as a lawsuit".to_string(),
        date_exhausted:"14th December,2026".to_string()
    };
    let task2 = Tasking{
        name:"Fredrick".to_string(),
        age:54,
        occupation:"Banker".to_string(),
        date_created:"15th May, 2023".to_string(),
        task:"To get a checkup before the holidays".to_string(),
        date_exhausted:"24th June,2023".to_string()
        };
    let task3 = Tasking{
        name:"Joseph".to_string(),
        age:21,
        occupation:"CyberSecurity developer".to_string(),
        date_created:"23rd January, 2026".to_string(),
        task:"To create a malware that can destroy Microsoft's Security".to_string(),
        date_exhausted:"15th March,2026".to_string()
    };
    println!("{}",task3.name);
    let mut tp = Vec::new();
    tp.push(task1);
    tp.push(task2);
    tp.push(task3);
     for i in 0..3{
        println!("Name:{:?}\nAge:{:?}\nOccupation:{:?}\nDate created:{:?}\nTask:{:?}\nDate exhausted:{:?}\n",tp[i].name,tp[i].age,tp[i].occupation,tp[i].date_created,tp[i].task,tp[i].date_exhausted);
     }

}
