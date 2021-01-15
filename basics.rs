// in order to obtain user input, we need to leverage the standard library io package
use std::io;

fn main() {
  // obtaining user input for a string
  let mut a = String::new();
  println!("Enter a string");
  // use the io package functions to acquire the input, then 
  // assign that to the previously declared variable using a reference to it (&mut a)
  io::stdin().read_line(&mut a).expect("Failed");
  println!("{}", a);

  // obtaining user input in all other data types:
  /* 
  All user input in Rust is read as a string; in order to save input as any other
  data type, it must be cast to that data type
  */

  let mut b = String::new();
  println!("Enter a number");
  io::stdin().read_line(&mut b).expect("Failed");
  // .trim() removes the new line \n from a user input
  // \n, \t can be used in the println! by being included directly in the quotes
  let b:i32 = b.trim().parse().expect("Failed");
  let c:i32 = b + 47129;
  println!("{}", b);
  println!("c, which is an i32 data type, whose value is the value of b (above) + 47129 is {}\n", c);
  
  // this same approach is used for integers, floats, and booleans
  
  let mut d = String::new();
  println!("Enter a boolean value \n");
  io::stdin().read_line(&mut d).expect("Failed");
  let d:bool = d.trim().parse().expect("Failed");
  println!("{}", d);


  // conditional statements

  // odd or even
  if c % 2 == 0 {
    println!("Even");
  } else {
    println!("Odd");
  };

  // user input dictating output
  let mut ch = String::new();
  println!("Are your friends coming to the movie?");
  io::stdin().read_line(&mut ch).expect("Failed");
  // the \n needs to be removed from the user response and the string splice needs to be converted
  // to a string in order to perform the boolean comparison
  ch = ch.trim().to_string();

  if ch == "y" {
    println!("Who's buying tickets?");
  } else {
    println!("Netflix it is!");
  };
 
  // find greatest number between two options
  let e:f32 = 4.87;
  let f:f32 = 23.125465;
  if e > f {
    println!("e is greater than f");
  } else {
    println!("f is greater than e");
  };


}