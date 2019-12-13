use std::convert::TryInto;
fn main() 
{
  println!("Enter a year");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input);
  let value: u32 = input.trim().parse().unwrap();
  let check = is_leap(value.try_into().unwrap());
  if check == true
  {
    println!("{} is a leap year", value);
  }
  else
  {
    println!("{} is not a leap year", value);
  }
}
// add an extra day every 4 years
// skip it if it's a new century 
//unless the century is divisible by 400
fn is_leap(x: i32)-> bool
{
  if x%400==0
  {
    return true;
  }
  if x%100==0
  {
    return false;
  }
  if x%4==0
  {
    return true;
  }
  return false;
}
