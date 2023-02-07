// Option
// enum Option<T>{
//   None, 
//   Some(T),
// }

// Result
// OCaml



fn take_fifth(value: Vec<i32>) -> Option<i32> {
  if value.len() < 5 { 
    None
  } else {
    Some(value[4]) // i32
  }
}

fn main(){
  let new_vec = vec![1,2];
  let new_long_vec = vec![1,2,3,4,5,6,7,8];
  let new_index = take_fifth(new_vec);
  let new_long_index = take_fifth(new_long_vec);
  pringln!("{:?}", new_index); //None
  pringln!("{:?}", new_long_index);

  match new_index {
    Some(number) => println!("I got a number: {}", number),
    None => println!("There was nothing inside")
  }

  match new_long_index {
    Some(number) => println!("I got a number: {}", number),
    None => println!("There was nothing inside")
  }
}