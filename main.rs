use std::convert::TryInto;
fn main()  
{
  println!("Enter a number");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input);
  let value: u32 = input.trim().parse().unwrap();
  let check = is_armstrong(value.try_into().unwrap());
  if check == true
  {
    println!("{} is an armstrong number", value);
  }
  else
  {
    println!("{} is not an armstrong number", value);
  }
}

fn digits(x: i32)-> i32 
{
  let mut counter=0;
  let mut y=x;
  while y>0 
  {
    counter = counter + 1;
    y=y/10;
  } 
  return counter;
}

fn is_armstrong(x: i32)-> bool
{
  let mut y = x;
  let mut sum = 0;
  let digits = digits(x);
  let max = digits + 1;
  for _i in 0..max 
  {
    let t = y%10;
    y = y/10;
    sum = sum + t.pow(digits as u32);
  }
  if sum == x
  {
    return true;
  }
  else
  {
    return false;
  }
}