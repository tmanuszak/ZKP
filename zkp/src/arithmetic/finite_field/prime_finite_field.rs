use rug::Integer;

#[derive(Debug, PartialEq)]
pub struct PrimeFiniteField {
    pub modulus: Integer,
}

impl PrimeFiniteField {
    fn new(modulus: Integer) -> Result<Self, &'static str> {
        // TODO: check modulus is prime
        Ok(PrimeFiniteField { modulus })
    }
}
