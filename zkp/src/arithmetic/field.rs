#![allow(dead_code)]

use rug::Integer;
use std::any::TypeId;

#[derive(Debug)]
pub enum FiniteField {
    PrimeFiniteField(PrimeFiniteField),
    NonPrimeFiniteField(NonPrimeFiniteField),
}

#[derive(Debug)]
pub struct PrimeFiniteField {
    modulus: Integer,
}

impl PrimeFiniteField {
    fn new(modulus: Integer) -> Result<Self, &'static str> {
        // TODO: check modulus is prime
        Ok(PrimeFiniteField { modulus })
    }
}

#[derive(Debug)]
pub struct NonPrimeFiniteField {
    order: Integer,
    characteristic: Integer,
    degree: Integer,
}

impl NonPrimeFiniteField {
    fn new(order: Integer, characteristic: Integer, degree: Integer) -> Result<Self, &'static str> {
        // TODO: check if characteristic is prime and if order == characteristic ^ degree
        Ok(NonPrimeFiniteField {
            order,
            characteristic,
            degree,
        })
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use rug::Integer;

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
}
