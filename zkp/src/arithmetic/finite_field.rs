#![allow(dead_code)]

pub mod non_prime_finite_field;
pub mod prime_finite_field;

use non_prime_finite_field::NonPrimeFiniteField;
use prime_finite_field::PrimeFiniteField;

#[derive(Debug, PartialEq)]
pub enum FiniteField {
    PrimeFiniteField(PrimeFiniteField),
    NonPrimeFiniteField(NonPrimeFiniteField),
}
