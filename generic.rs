// generics <-> conrete
// use std::fmt::Display;
// fn give_thing<T: Display>(input:T) -> T{
//   println!("{}",input);
//   input;
// }

// fn give_thing<GenericType>(input:GenericType) -> GenericType{
//   input;
// }

// fn main() {
//   let x = give_thing(String::from("Take this thing"));
//   let y = give_thing(9);

//   println!("{}",x);
//   println!("{}",y);
// }

use std::fmt::Display;
use std::cmp::PartialOrd;

// fn compare_and_print<T: Display, U: Display+PartialOrd>(statement: T,num_1: U, num_2: U){
//   println!("{}! is {} greater than {}? {}", statement, num_1, num_2, num1 > num_2);
// }

fn compare_and_print<DisplayType, CompareType>(
  statement: DisplayType,
  num_1: CompareType,
  num_2: CompareType
)where
  DisplayType: Display, 
  CompareType: Display+PartialOrd
{
  println!("{}! is {} greater than {}? {}", statement, num_1, num_2, num1 > num_2);
}


fn main (){
  compare_and_print("Listen up!", 9, 8);
}