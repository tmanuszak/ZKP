pub mod finite_field;
pub mod finite_field_element;
pub mod polynomial;
pub mod utils;

// For refactoring the arbitrary precision arithmetic in the future
type Integer = rug::Integer;