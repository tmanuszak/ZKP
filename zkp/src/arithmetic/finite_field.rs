#![allow(dead_code)]

pub mod non_prime_finite_field;
pub mod prime_finite_field;

pub use non_prime_finite_field::NonPrimeFiniteField;
pub use prime_finite_field::PrimeFiniteField;

// Instead of a box, make an enum to keep on the stack
#[derive(Debug, PartialEq)]
pub enum FiniteField {
    PrimeFiniteFieldVariant(PrimeFiniteField),
    NonPrimeFiniteFieldVariant(NonPrimeFiniteField),
}

pub use FiniteField::{PrimeFiniteFieldVariant, NonPrimeFiniteFieldVariant};
