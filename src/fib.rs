// Two ways to solve the fibinacci challenge 

const N: i32 = 123;

pub fn run() {
  let mut x = 0;
  println!("The N=0 factioal is {}", x); 
  let mut y = 1;
  println!("The N=1 factorial is {}", y); 
  let mut z = x + y; 
  println!("The N=2 factorial is {}", z);
  for i in 3..N+1 {
    x = y;
    y = z; 
    z = x + y;
    println!(" The N= {} factorial is {}", i, z);     
  }

}


//const N: u32 = 123;

//pub fn run(n:u128) ->u128 {
  //  if n == 0 {
    //  0
    //} else if n == 1 {
      //1
   // } else {
     // fib(n-1) + fib(n-2)
    //}
  
//}

//fn fib() {
  //for i in 0..N+1 {
    //println!("The N= {}  factiorial is {}", i, fib(i.into())); 
//  }
//}