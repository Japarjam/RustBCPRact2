//test a number to see if it is prime. eventually use std::io to get a number inputted by user. 

const N: u128 = 123213243241; 

fn is_divisable(n: u128) -> bool  {
    for i in 2..n/2 {
      if n%i == 0 {
        return true; 
      }
    }
  false
}

pub fn run() {
  if is_divisable(N) {
    println!("N={} is NOT prime", N);
  } else {
    println!("N={} is prime", N); 
  }
}