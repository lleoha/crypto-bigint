//! [`Int`] subtraction operations.

use core::ops::{Sub, SubAssign};
use subtle::{ConstantTimeEq, CtOption};
use crate::{Checked, CheckedSub, Int};

impl<const LIMBS: usize> CheckedSub for Int<LIMBS> {
    fn checked_sub(&self, rhs: &Self) -> CtOption<Self> {
        // Step 1. subtract operands
        let res = Self(self.0.wrapping_sub(&rhs.0));

        // Step 2. check whether underflow happened.
        // Note:
        // - underflow can only happen when the inputs have opposing signs, and then
        // - underflow occurs if and only if the result and the lhs have opposing signs.
        //
        // We can thus express the overflow flag as: (self.msb != rhs.msb) & (self.msb != res.msb)
        let self_msb = self.sign_bit();
        let underflow = self_msb.ct_ne(&rhs.sign_bit()) & self_msb.ct_ne(&res.sign_bit());

        // Step 3. Construct result
        CtOption::new(
            res,
            !underflow,
        )
    }
}

impl<const LIMBS: usize> Sub for Int<LIMBS> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self.sub(&rhs)
    }
}

impl<const LIMBS: usize> Sub<&Int<LIMBS>> for Int<LIMBS> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self {
        self.checked_sub(rhs)
            .expect("attempted to subtract with underflow")
    }
}


impl<const LIMBS: usize> SubAssign for Checked<Int<LIMBS>> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<const LIMBS: usize> SubAssign<&Checked<Int<LIMBS>>> for Checked<Int<LIMBS>> {
    fn sub_assign(&mut self, other: &Self) {
        *self = *self - other;
    }
}


#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod tests {
        use crate::{CheckedSub, Int, U128};
        use crate::int::I128;

        #[test]
        fn checked_sub() {
            let min_plus_one = Int{ 0: I128::MIN.0.wrapping_add(&I128::ONE.0) };
            let max_minus_one = Int{ 0: I128::MAX.0.wrapping_sub(&I128::ONE.0) };
            let two = Int{ 0: U128::from(2u32) };
            let min_plus_two = Int{ 0: I128::MIN.0.wrapping_add(&two.0) };

            // lhs = MIN

            let result = I128::MIN.checked_sub(&I128::MIN);
            assert_eq!(result.unwrap(), I128::ZERO);

            let result = I128::MIN.checked_sub(&I128::MINUS_ONE);
            assert_eq!(result.unwrap(), min_plus_one);

            let result = I128::MIN.checked_sub(&I128::ZERO);
            assert_eq!(result.unwrap(), I128::MIN);

            let result = I128::MIN.checked_sub(&I128::ONE);
            assert!(bool::from(result.is_none()));

            let result = I128::MIN.checked_sub(&I128::MAX);
            assert!(bool::from(result.is_none()));

            // lhs = -1

            let result = I128::MINUS_ONE.checked_sub(&I128::MIN);
            assert_eq!(result.unwrap(), I128::MAX);

            let result = I128::MINUS_ONE.checked_sub(&I128::MINUS_ONE);
            assert_eq!(result.unwrap(), I128::ZERO);

            let result = I128::MINUS_ONE.checked_sub(&I128::ZERO);
            assert_eq!(result.unwrap(), I128::MINUS_ONE);

            let result = I128::MINUS_ONE.checked_sub(&I128::ONE);
            assert_eq!(result.unwrap(), two.neg().unwrap());

            let result = I128::MINUS_ONE.checked_sub(&I128::MAX);
            assert_eq!(result.unwrap(), I128::MIN);

            // lhs = 0

            let result = I128::ZERO.checked_sub(&I128::MIN);
            assert!(bool::from(result.is_none()));

            let result = I128::ZERO.checked_sub(&I128::MINUS_ONE);
            assert_eq!(result.unwrap(), I128::ONE);

            let result = I128::ZERO.checked_sub(&I128::ZERO);
            assert_eq!(result.unwrap(), I128::ZERO);

            let result = I128::ZERO.checked_sub(&I128::ONE);
            assert_eq!(result.unwrap(), I128::MINUS_ONE);

            let result = I128::ZERO.checked_sub(&I128::MAX);
            assert_eq!(result.unwrap(), min_plus_one);

            // lhs = 1

            let result = I128::ONE.checked_sub(&I128::MIN);
            assert!(bool::from(result.is_none()));

            let result = I128::ONE.checked_sub(&I128::MINUS_ONE);
            assert_eq!(result.unwrap(), two);

            let result = I128::ONE.checked_sub(&I128::ZERO);
            assert_eq!(result.unwrap(), I128::ONE);

            let result = I128::ONE.checked_sub(&I128::ONE);
            assert_eq!(result.unwrap(), I128::ZERO);

            let result = I128::ONE.checked_sub(&I128::MAX);
            assert_eq!(result.unwrap(), min_plus_two);

            // lhs = MAX

            let result = I128::MAX.checked_sub(&I128::MIN);
            assert!(bool::from(result.is_none()));

            let result = I128::MAX.checked_sub(&I128::MINUS_ONE);
            assert!(bool::from(result.is_none()));

            let result = I128::MAX.checked_sub(&I128::ZERO);
            assert_eq!(result.unwrap(), I128::MAX);

            let result = I128::MAX.checked_sub(&I128::ONE);
            assert_eq!(result.unwrap(), max_minus_one);

            let result = I128::MAX.checked_sub(&I128::MAX);
            assert_eq!(result.unwrap(), I128::ZERO);
        }
    }
}