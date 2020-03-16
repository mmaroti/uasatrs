/*
* Copyright (C) 2020, Miklos Maroti
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use super::boolalg::BoolAlg;
use super::genvec::{GenElem, GenVec};

/// Boolean array algebra.
pub trait BoolVecAlg {
    type Elem: Clone;

    /// Returns the length of the array.
    fn len(elem: &Self::Elem) -> usize;

    /// Creates a new vector of the given length containing the element.
    fn bit_lift(self: &Self, elem: &[bool]) -> Self::Elem;

    /// Returns the element wise negation of the vector.
    fn bit_not(self: &mut Self, elem: &Self::Elem) -> Self::Elem;

    /// Returns a new vector whose elements are disjunctions of the original
    /// elements.
    fn bit_or(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns a new vector whose elements are conjunction of the original
    /// elements.
    fn bit_and(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns a new vector whose elements are the exclusive or of the
    /// original elements.
    fn bit_add(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns a new vector whose elements are the are the logical equivalence
    /// of the original elements.
    fn bit_equ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns a new vector whose elements are the are the logical implication
    /// of the original elements.
    fn bit_leq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Concatenates the given vectors into a single one.
    fn concat(self: &Self, elems: &[&Self::Elem]) -> Self::Elem;

    /// Creates a new vector of the given length representing the given binary
    /// number.
    fn num_lift(self: &Self, len: usize, elem: i64) -> Self::Elem;

    /// Returns the negative of the given binary number in two's complement.
    fn num_neg(self: &mut Self, elem: &Self::Elem) -> Self::Elem;

    /// Returns the sum of the two binary numbers of the same length in
    /// two's complement.
    fn num_add(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;

    /// Returns the difference of the two binary numbers of the same length in
    /// two's complement.
    fn num_sub(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem;
}

pub type Checker = ();

impl BoolVecAlg for Checker {
    type Elem = usize;

    fn len(elem: &Self::Elem) -> usize {
        *elem
    }

    fn bit_lift(self: &Self, elem: &[bool]) -> Self::Elem {
        elem.len()
    }

    fn bit_not(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        *elem
    }

    fn bit_or(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(*elem1 == *elem2);
        *elem1
    }

    fn bit_and(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(*elem1 == *elem2);
        *elem1
    }

    fn bit_add(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(*elem1 == *elem2);
        *elem1
    }

    fn bit_equ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(*elem1 == *elem2);
        *elem1
    }

    fn bit_leq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(*elem1 == *elem2);
        *elem1
    }

    fn concat(self: &Self, elems: &[&Self::Elem]) -> Self::Elem {
        elems.iter().fold(0, |sum, elem| sum + *elem)
    }

    fn num_lift(self: &Self, len: usize, _elem: i64) -> Self::Elem {
        len
    }

    fn num_neg(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        *elem
    }

    fn num_add(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(*elem1 == *elem2);
        *elem1
    }

    fn num_sub(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(*elem1 == *elem2);
        *elem1
    }
}

impl<ALG> BoolVecAlg for ALG
where
    ALG: BoolAlg,
{
    type Elem = <ALG::Elem as GenElem>::Vector;

    fn len(elem: &Self::Elem) -> usize {
        elem.len()
    }

    fn bit_lift(self: &Self, elem: &[bool]) -> Self::Elem {
        GenVec::from_fn(elem.len(), |i| self.bool_lift(elem[i]))
    }

    fn bit_not(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        GenVec::from_fn(elem.len(), |i| self.bool_not(elem.get(i)))
    }

    fn bit_or(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        GenVec::from_fn(elem1.len(), |i| self.bool_or(elem1.get(i), elem2.get(i)))
    }

    fn bit_and(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        GenVec::from_fn(elem1.len(), |i| self.bool_and(elem1.get(i), elem2.get(i)))
    }

    fn bit_add(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        GenVec::from_fn(elem1.len(), |i| self.bool_add(elem1.get(i), elem2.get(i)))
    }

    fn bit_equ(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        GenVec::from_fn(elem1.len(), |i| self.bool_equ(elem1.get(i), elem2.get(i)))
    }

    fn bit_leq(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        GenVec::from_fn(elem1.len(), |i| self.bool_leq(elem1.get(i), elem2.get(i)))
    }

    fn concat(self: &Self, elems: &[&Self::Elem]) -> Self::Elem {
        let size = elems.iter().fold(0, |sum, elem| sum + elem.len());
        let mut result: Self::Elem = GenVec::with_capacity(size);
        for elem in elems {
            result.extend(elem);
        }
        result
    }

    fn num_lift(self: &Self, len: usize, elem: i64) -> Self::Elem {
        GenVec::from_fn(len, |i| self.bool_lift((elem >> i) & 1 != 0))
    }

    fn num_neg(self: &mut Self, elem: &Self::Elem) -> Self::Elem {
        let mut carry = self.bool_unit();
        let mut result: Self::Elem = GenVec::with_capacity(elem.len());
        for i in 0..elem.len() {
            let not_elem = self.bool_not(elem.get(i));
            result.push(self.bool_add(not_elem, carry));
            carry = self.bool_and(not_elem, carry);
        }
        result
    }

    fn num_add(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        let mut carry = self.bool_zero();
        let mut result: Self::Elem = GenVec::with_capacity(elem1.len());
        for i in 0..elem1.len() {
            result.push(self.bool_ad3(elem1.get(i), elem2.get(i), carry));
            carry = self.bool_maj(elem1.get(i), elem2.get(i), carry);
        }
        result
    }

    fn num_sub(self: &mut Self, elem1: &Self::Elem, elem2: &Self::Elem) -> Self::Elem {
        assert!(elem1.len() == elem2.len());
        let mut carry = self.bool_unit();
        let mut result: Self::Elem = GenVec::with_capacity(elem1.len());
        for i in 0..elem1.len() {
            let not_elem2 = self.bool_not(elem2.get(i));
            result.push(self.bool_ad3(elem1.get(i), not_elem2, carry));
            carry = self.bool_maj(elem1.get(i), not_elem2, carry);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::super::boolalg::Boolean;
    use super::*;

    #[test]
    fn num_lift() {
        let alg = Boolean::new();
        let v1 = alg.bit_lift(&[true, false, true, true]);
        let v2 = alg.num_lift(4, 13);
        assert_eq!(v1, v2);

        let v3 = alg.num_lift(4, -3);
        assert_eq!(v1, v3);
    }

    #[test]
    fn concat() {
        let alg = Boolean::new();
        let v1 = alg.num_lift(4, 0x3);
        let v2 = alg.num_lift(4, 0xd);
        let v3 = alg.num_lift(8, 0xd3);
        let v4 = alg.concat(&[&v1, &v2]);
        assert_eq!(v3, v4);
    }

    #[test]
    fn numops() {
        let mut alg = Boolean::new();
        let v1 = alg.num_lift(8, 17);
        let v2 = alg.num_lift(8, 73);
        let v3 = alg.num_lift(8, 90);
        let v4 = alg.num_add(&v1, &v2);
        assert_eq!(v3, v4);
        let v5 = alg.num_lift(8, -17);
        let v6 = alg.num_neg(&v1);
        assert_eq!(v5, v6);
        let v7 = alg.num_lift(8, -56);
        let v8 = alg.num_sub(&v1, &v2);
        assert_eq!(v7, v8);
    }
}
