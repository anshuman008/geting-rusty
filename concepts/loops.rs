  
use std::io;

  fn main(){

    // making the logic of guessing game

    let mut guess = String::new();
    let target = 34;
    let mut counts = 0;

     println!("game started!!");

     println!("guess a number!!");

     loop {
         io::stdin().read_line(&mut guess).expect("please enter a valid input!!");
     
         let userinput:i32 = guess.trim().parse().expect("please enter a valid number!!");
     
         if target == userinput{
             println!("wohhh you won the game!! in {} attempts",counts);
             break;
         }
         else if target > userinput{
            println!("guess a higher number");
         }
         else if target < userinput {
            println!("guess a lower number");
         }


         guess.clear();
         counts += 1;
     }

  }