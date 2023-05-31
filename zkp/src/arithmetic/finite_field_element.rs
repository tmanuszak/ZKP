// TODO: Maybe hide the Value type and redo the new method for FiniteFieldElement to just take a Vec<Integer> or Integer?

#![allow(dead_code)]

use crate::finite_field::FiniteField;
use rug::Integer;
use std::ops::{Add, Rem};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Value {
    PrimeFieldValue(Integer),
    NonPrimeFieldValue(Vec<Integer>),
}

#[derive(Debug)]
pub struct FiniteFieldElement {
    field: Rc<FiniteField>, // Many elements have the same finite field, so we can share it.
    value: Value,
}

impl FiniteFieldElement {
    fn new(field: &Rc<FiniteField>, value: Value) -> Result<Self, &'static str> {
        match (&**field, value) {
            (FiniteField::PrimeFiniteField(_), Value::PrimeFieldValue(a)) => {
                Ok(FiniteFieldElement {
                    field: Rc::clone(&field),
                    value: Value::PrimeFieldValue(Integer::from(a)),
                })
            }
            (FiniteField::NonPrimeFiniteField(_), Value::NonPrimeFieldValue(a)) => {
                Ok(FiniteFieldElement {
                    field: Rc::clone(&field),
                    value: Value::NonPrimeFieldValue(a),
                })
            }
            (FiniteField::PrimeFiniteField(_), Value::NonPrimeFieldValue(_)) => {
                Err("value must be an Integer")
            }
            (FiniteField::NonPrimeFiniteField(_), Value::PrimeFieldValue(_)) => {
                Err("value must be a vector of Integer")
            }
        }
    }
}

struct MyStruct {
    value: Integer,
}

impl Add for MyStruct {
    type Output = MyStruct;

    fn add(self, other: MyStruct) -> MyStruct {
        let sum = self.value + other.value;
        MyStruct { value: sum }
    }
}

// TODO: Fix so that it is self + other % modulus
impl<'a, 'b> Add<&'b FiniteFieldElement> for &'a FiniteFieldElement {
    type Output = FiniteFieldElement;
    fn add(self, other: &'b FiniteFieldElement) -> FiniteFieldElement {
        if *self.field != *other.field {
            println!(
                "&self.field = {:?},\n&other.field = {:?}\n",
                &self.field, &other.field
            );
            panic!("Cannot add elements from different finite fields");
        }
        match (&*self.field, &self.value, &other.value) {
            (
                FiniteField::PrimeFiniteField(prime_field),
                Value::PrimeFieldValue(a),
                Value::PrimeFieldValue(b),
            ) => FiniteFieldElement {
                field: Rc::clone(&self.field),
                value: Value::PrimeFieldValue(Integer::from(a + b).rem(&prime_field.modulus)),
            },
            (
                FiniteField::NonPrimeFiniteField(_),
                Value::NonPrimeFieldValue(_),
                Value::NonPrimeFieldValue(_),
            ) => unimplemented!(),
            (_, _, _) => {
                println!(
                    "&*self = {:?},\n&self.value = {:?},\n&other.value = {:?}\n",
                    &*self, &self.value, &other.value
                );
                panic!("Cannot add elements from different finite fields");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finite_field::non_prime_finite_field::NonPrimeFiniteField;
    use crate::finite_field::prime_finite_field::PrimeFiniteField;
    use rug::Integer;

    #[test]
    fn test_new_fieldelement_prime() -> Result<(), String> {
        let f = Rc::new(FiniteField::PrimeFiniteField(PrimeFiniteField {
            modulus: Integer::from(5),
        }));
        let a = FiniteFieldElement::new(&f, Value::PrimeFieldValue(Integer::from(3)))
            .expect("failed to create finite field element a");
        println!("f: {:?}", f);
        println!("a: {:?}", a);
        Ok(())
    }

    #[test]
    fn test_new_fieldelement_nonprime() -> Result<(), String> {
        let f = Rc::new(FiniteField::NonPrimeFiniteField(NonPrimeFiniteField {
            order: Integer::from(6),
            characteristic: Integer::from(3),
            degree: Integer::from(2),
        }));
        let a = FiniteFieldElement::new(
            &f,
            Value::NonPrimeFieldValue(Vec::from([
                Integer::from(1),
                Integer::from(2),
                Integer::from(3),
            ])),
        )
        .expect("failed to create finite field element a");
        println!("f: {:?}", f);
        println!("a: {:?}", a);
        Ok(())
    }

    #[test]
    fn test_add_fieldelement_prime() -> Result<(), String> {
        let f = Rc::new(FiniteField::PrimeFiniteField(PrimeFiniteField {
            modulus: Integer::from(5),
        }));
        let a = FiniteFieldElement::new(&f, Value::PrimeFieldValue(Integer::from(3)))
            .expect("failed to create finite field element a");
        let b = FiniteFieldElement::new(&f, Value::PrimeFieldValue(Integer::from(4)))
            .expect("failed to create finite field element b");
        let c = &a + &b;
        println!("f: {:?}", f);
        println!("a: {:?}", a);
        println!("b: {:?}", b);
        println!("c: {:?}", c);
        Ok(())
    }
}
