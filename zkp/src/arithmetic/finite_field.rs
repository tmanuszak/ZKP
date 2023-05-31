#![allow(dead_code)]

pub mod prime_finite_field;
pub mod non_prime_finite_field;

use prime_finite_field::PrimeFiniteField;
use non_prime_finite_field::NonPrimeFiniteField;

#[derive(Debug, PartialEq)]
pub enum FiniteField {
    PrimeFiniteField(PrimeFiniteField),
    NonPrimeFiniteField(NonPrimeFiniteField),
}
