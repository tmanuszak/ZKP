// TODO: Maybe hide the Value type and redo the new method for FiniteFieldElement to just take a Vec<Integer> or Integer?

#![allow(dead_code)]

use crate::arithmetic::Integer;
use crate::finite_field::FiniteField; // importing the FF enum
use crate::finite_field::FiniteField::{NonPrimeFiniteFieldVariant, PrimeFiniteFieldVariant};  // importing the FF variants so it is cleaner to refer to
use std::ops::{Add, Rem};  // For operator overloading
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Value {
    PrimeFieldValue(Integer),
    NonPrimeFieldValue(Vec<Integer>),
}

use Value::{NonPrimeFieldValue, PrimeFieldValue};

#[derive(Debug)]
pub struct FiniteFieldElement {
    field: Rc<FiniteField>, // Many elements have the same finite field, so we can share it.
    value: Value,
}

impl FiniteFieldElement {
    fn new(field: &Rc<FiniteField>, value: Value) -> Result<Self, &'static str> {
        match (&**field, value) {
            (PrimeFiniteFieldVariant(_), PrimeFieldValue(a)) => Ok(FiniteFieldElement {
                field: Rc::clone(&field),
                value: PrimeFieldValue(Integer::from(a)),
            }),
            (NonPrimeFiniteFieldVariant(_), NonPrimeFieldValue(a)) => Ok(FiniteFieldElement {
                field: Rc::clone(&field),
                value: NonPrimeFieldValue(a),
            }),
            (PrimeFiniteFieldVariant(_), NonPrimeFieldValue(_)) => Err("value must be an Integer"),
            (NonPrimeFiniteFieldVariant(_), PrimeFieldValue(_)) => {
                Err("value must be a vector of Integer")
            }
        }
    }
}

// TODO: Fix so that it is self + other % modulus
// This will consume both the operands.
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
            (PrimeFiniteFieldVariant(prime_field), PrimeFieldValue(a), PrimeFieldValue(b)) => {
                FiniteFieldElement {
                    field: Rc::clone(&self.field),
                    value: PrimeFieldValue(Integer::from(a + b).rem(&prime_field.modulus)),
                }
            }
            (NonPrimeFiniteFieldVariant(_), NonPrimeFieldValue(_), NonPrimeFieldValue(_)) => {
                unimplemented!()
            }
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
    use crate::finite_field::{PrimeFiniteField, NonPrimeFiniteField};

    #[test]
    fn test_new_fieldelement_prime() -> Result<(), String> {
        let field = Rc::new(PrimeFiniteFieldVariant(PrimeFiniteField {
            modulus: Integer::from(5),
        }));
        let element = FiniteFieldElement::new(
            &field,  // Field
            PrimeFieldValue(Integer::from(3)),  // Value
        )
        .expect("failed to create finite field element a");
        println!("FiniteFieldElement: {}", std::mem::size_of::<FiniteFieldElement>());
        println!("Value: {}", std::mem::size_of::<Value>());
        println!("FiniteField: {}", std::mem::size_of::<FiniteField>());
        println!("PrimeFiniteField: {}", std::mem::size_of::<PrimeFiniteField>());
        println!("NonPrimeFiniteField: {}", std::mem::size_of::<NonPrimeFiniteField>());
        println!("Integer: {}", std::mem::size_of::<Integer>());
        println!("Vec<Integer>: {}", std::mem::size_of::<Vec<Integer>>());
        println!("Rc<FiniteField>: {}", std::mem::size_of::<Rc<FiniteField>>());
        println!("element: {:?}", element);
        Ok(())
    }

    // #[test]
    // fn test_new_fieldelement_nonprime() -> Result<(), String> {
    //     let field = Rc::new(NonPrimeFiniteFieldVariant(NonPrimeFiniteField {
    //         order: Integer::from(6),
    //         characteristic: Integer::from(3),
    //         degree: Integer::from(2),
    //     }));
    //     let element = FiniteFieldElement::new(
    //         &field,
    //         NonPrimeFieldValue(Vec::from([
    //             Integer::from(1),
    //             Integer::from(2),
    //             Integer::from(3),
    //         ])),
    //     )
    //     .expect("failed to create finite field element a");
    //     println!("field: {:?}", field);
    //     println!("element: {:?}", element);
    //     Ok(())
    // }

    // #[test]
    // fn test_add_fieldelement_prime() -> Result<(), String> {
    //     let field = Rc::new(PrimeFiniteFieldVariant(PrimeFiniteField {
    //         modulus: Integer::from(5),
    //     }));
    //     let a = FiniteFieldElement::new(&field, PrimeFieldValue(Integer::from(3)))
    //         .expect("failed to create finite field element a");
    //     let b = FiniteFieldElement::new(&field, PrimeFieldValue(Integer::from(4)))
    //         .expect("failed to create finite field element b");
    //     let c = &a + &b;
    //     println!("f: {:?}", field);
    //     println!("a: {:?}", a);
    //     println!("b: {:?}", b);
    //     println!("c: {:?}", c);
    //     Ok(())
    // }
}
