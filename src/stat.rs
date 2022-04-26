pub fn run() {

  let array = [1,5,3,21,3,34,5,21,4,5,43,23,12,4];
  let mut min = 1000000; 
  let mut max: i32 = -1;
  let mut mean: f32 = 0.;
  for i in array.iter() {
    let i = *i;
    if i < min {
      min = i; 
    }
    if i > max {
      max = i;
    }
    mean += i as f32;
  }
  mean /= array.len() as f32;
  println!("The min is {}", min); 
  println!("The max is {}", max);
  println!("The mean is {}", mean); 

  }
 
 