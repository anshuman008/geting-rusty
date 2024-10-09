// now we are going to learn about functions in rust 
use std::io;


fn main(){
    

    let mut name =  String::new();

    println!("hey whats your name!!");
    io::stdin().read_line(&mut name).expect("failed during taking the input");

    greet(name);


    let inputs = takeinput();

    println!("here is sum of a {} and b {} a+b = {}",inputs.0,inputs.1,inputs.0+inputs.1);

    describe_person("anshuman".to_string(),20,5.8)
}




// greet function
fn greet(name:String){
    println!("hello {name} kaise ho aap sir!!");
}


// function to take two numbers as input
fn takeinput() -> (i8,i8){

    let mut input = String::new(); // temp buffer
   

    println!("enter the first number");
    io::stdin().read_line(&mut input).expect("error while taking input");
 
      let a:i8 = input.trim().parse().expect("error while parsing the input");
      

      // clearing the input for next input;
      input.clear();

      println!("enter the second number");
      io::stdin().read_line(&mut input).expect("error while taking the input");

      let b:i8 = input.trim().parse().expect("error whiel parsing the  input");
    
     (a,b)
}


// function to describe_person
fn describe_person(name:String,age:u8,height:f32){
    println!("my name is {} and im {} years old and my height is {}",name,age,height);
}