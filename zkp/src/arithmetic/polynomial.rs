// use field::{FieldElement, FiniteField};

// pub struct UnivariatePolynomial {
//     coefficients: Vec<FieldElement>,
// }

// pub struct MultivariatePolynomial {
//     coefficients: Vec<Vec<FieldElement>>,
// }

// impl UnivariatePolynomial {
//     pub fn new(coefficients: Vec<FieldElement>) -> Self {
//         UnivariatePolynomial { coefficients }
//     }

//     // Figure this out
//     pub fn evaluate(&self, x: &FieldElement) -> FieldElement {
//         let mut result = 0.0;
//         for (i, coefficient) in self.coefficients.iter().enumerate() {
//             result += coefficient * x.pow(i as u32);
//         }
//         result
//     }
// }

// impl MultivariatePolynomial {
//     pub fn new(coefficients: Vec<Vec<FieldElement>>) -> Self {
//             MultivariatePolynomial { coefficients }
//         }

//     // Figure this out
//     pub fn evaluate(&self, x: Vec<FieldElement>) -> Vec<FieldElement> {
//         let mut result = vec![0.0; self.coefficient[0].len()];
//         for (i, coefficients) in self.coefficients.iter().enumerate() {
//             for (j, coefficient) in coefficients.iter().enumerate() {
//                 result[i] += coefficient * x[j].pow(i as u32);
//             }
//         }
//         result
//     }
// }
