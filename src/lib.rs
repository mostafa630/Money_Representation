use core::fmt;
use std::{marker::PhantomData, ops::Add};

pub trait Currency {
    const SYMBOL : &'static str ;
    const RATIO : u8;
}
pub struct USD;
impl Currency for USD {
    const SYMBOL : &'static str = "$";
    const RATIO : u8 = 100; // the minor unit (means dollar caontains 100 cent)
}
pub struct EGP;
impl Currency for EGP {
    const SYMBOL : &'static str = "LE";
    const RATIO : u8 = 100; // the minor unit (means Pound caontains 100 2rsh)
}
#[derive(Debug)]
pub struct Amount<C>{
    amount:i64 ,
    phantom : PhantomData<C>
}
impl<C:Currency> Amount<C> {                     //-currency only used to define type of C when passed
    pub fn new(value:i64 , _currency:C)->Self{   // thre is no other usage for it 
        Amount{
            amount:value,
            phantom:PhantomData
        }
    }
}
impl<C:Currency> fmt::Display for Amount<C> { // implement display for Amount of USD only
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f ,"{}.{}{}" ,self.amount / C::RATIO as i64 , self.amount%C::RATIO as i64 , C::SYMBOL)
    }
}
impl<C> std::ops::Add<Self> for Amount<C> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Amount {
             amount: self.amount + other.amount,
              phantom: PhantomData,
        }
    }
}

