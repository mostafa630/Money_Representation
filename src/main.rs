use Money_Representation::*;
fn main() {
   print!("#------>USA CURRENCY<------------#");
   let a= Amount::new( 55, USD);
   let b= Amount::new(146,USD);
   println!("value of a = {}",a);
   println!("value of b = {}",a);
   println!("value of sum(a,b) = {}",a +b);
   print!("#------>EGYPTION CURRENCY<------------#");
   let a = Amount::new(20 ,EGP);
   let b = Amount::new(562,EGP);
   println!("value of a = {}",a);
   println!("value of b = {}",a);
   println!("value of sum(a,b) = {}",a +b);
   
}
