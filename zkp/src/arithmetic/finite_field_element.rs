#![allow(dead_code)]

use rug::Integer;
use std::any::TypeId;
use std::ops::{Rem, Add};
use crate::finite_field::FiniteField;
use crate::finite_field::prime_finite_field::PrimeFiniteField;

#[derive(Debug)]
pub struct FiniteFieldElement<T> {
    field: FiniteField,
    value: T,
}

impl<T: 'static> FiniteFieldElement<T> {
    fn new(field: FiniteField, value: T) -> Result<FiniteFieldElement<T>, &'static str> {
        match field {
            FiniteField::PrimeFiniteField(field) => {
                if TypeId::of::<T>() == TypeId::of::<Integer>() {
                    return Ok(FiniteFieldElement {
                        field: FiniteField::PrimeFiniteField(field),
                        value,
                    });
                } else {
                    return Err("value must be an integer");
                }
            }
            FiniteField::NonPrimeFiniteField(field) => {
                if TypeId::of::<T>() == TypeId::of::<Vec<Integer>>() {
                    return Ok(FiniteFieldElement {
                        field: FiniteField::NonPrimeFiniteField(field),
                        value,
                    });
                } else {
                    return Err("value must be a vector of integers");
                }
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
impl<T> Add for FiniteFieldElement<T> 
where
    T: Add<Output = T> + Rem<Output = T> + Clone,
{
    type Output = FiniteFieldElement<T>;
    fn add(self, other: FiniteFieldElement<T>) -> FiniteFieldElement<T> {
        assert!(self.field == other.field);
        match self.field {
            FiniteField::PrimeFiniteField(field) => {
                let modulus = field.modulus;
                let value = self.value + other.value;
                FiniteFieldElement {
                    field: FiniteField::PrimeFiniteField(PrimeFiniteField { modulus }),
                    value,
                }
            }
            FiniteField::NonPrimeFiniteField(field) => {
                // TODO
                unimplemented!()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rug::Integer;
    use crate::finite_field::prime_finite_field::PrimeFiniteField;
    use crate::finite_field::non_prime_finite_field::NonPrimeFiniteField;

    #[test]
    fn test_new_fieldelement_prime() -> Result<(), String> {
        let a = FiniteFieldElement::new(
            FiniteField::PrimeFiniteField(PrimeFiniteField {
                modulus: Integer::from(5),
            }),
            Integer::from(3)
        );
        assert!(a.is_ok());
        println!("{:?}", a?);
        Ok(())
    }

    #[test]
    fn test_new_fieldelement_nonprime() -> Result<(), String> {
        let a = FiniteFieldElement::new(
            FiniteField::NonPrimeFiniteField(NonPrimeFiniteField {
                order: Integer::from(9),
                characteristic: Integer::from(3),
                degree: Integer::from(2),
            }),
            vec![Integer::from(2), Integer::from(3)],
        );
        assert!(a.is_ok());
        println!("{:?}", a?);
        Ok(())
    }

    #[test]
    fn test_add_fieldelement_prime() -> Result<(), String> {
        let a = FiniteFieldElement::new(
            FiniteField::PrimeFiniteField(PrimeFiniteField {
                modulus: Integer::from(5),
            }),
            Integer::from(3)
        )?;
        let b = FiniteFieldElement::new(
            FiniteField::PrimeFiniteField(PrimeFiniteField {
                modulus: Integer::from(5),
            }),
            Integer::from(4)
        )?;
        let c = a + b;
        println!("{:?}", c);
        Ok(())
    }
}