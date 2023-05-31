use rug::Integer;

#[derive(Debug, PartialEq)]
pub struct NonPrimeFiniteField {
    pub order: Integer,
    pub characteristic: Integer,
    pub degree: Integer,
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