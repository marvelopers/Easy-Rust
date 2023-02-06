enum ThingsInTheSky {
    Sun,    // 0
    Starsm, // 1
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun"),
        ThingsInTheSky::Stars => println!("I can see the stars"),
    }
}

// fn main() {
//   let time = 8;
//   let sky_state = create_skystate(time);
//   check_skystate(&sky_state);

//   check_skystate(&create_skystate(20));
// }

enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

fn match_mood(mood: &Mood) -> i32{
  // let happiness_level = match mood{
  //   Mood::Happy => 10, 
  //   Mood::Sleepy => 6, 
  //   Mood::NotBad => 7,
  //   Mood:: Angry => 2
  // }
  // happiness_level

  use Mood::*;
  match mood{
    Happy => 10, 
    Sleepy => 6, 
    NotBad => 7,
     Angry => 2
  }
}

// fn main() {
//   let my_mood = Mood::NotBad;
//   let happiness_level = match_mood($my_mood);
//   println!("Out of 1 to 10, my happiness is {}", happiness_level);
// }

enum Season{
  Spring, 
  Summer, 
  Autumn, 
  Winter,
}

// fn main (){
//   use Season::*;
//   let four_season = vec![Spring, Summer, Autumn, Winter]; //Vec<Season>
//   for season in four_season{
//     pringln!("The number is: {}", season as u32);
//   }
// }

enum Star{
 BrownDwarf = 10, 
 RedDwarf = 50,
 YellowStar = 100,
 RedGiant = 1000,
 DeadStar //1001 숫자를 지정하지 않을 경우, 바로 전에 숫자 +1
}

fn main(){
  use Star::*;
  let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];

  for star in starvec {
    match star as u32 {
      size id size <= 80 => println!("Not the biggest star: {}", size),
      size id size >= 80 => println!("Not the big star: {}", size),
      _ => println!("Some other star")
    }
  }

  println!("What about DeadStar? It is: {}", DeadStar as u32);
}