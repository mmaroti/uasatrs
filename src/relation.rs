/*
* Copyright (C) 2019-2020, Miklos Maroti
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

use super::boolean;
use super::tensor;

pub struct Universe<ALG>
where
    ALG: tensor::TensorAlg,
{
    alg: ALG,
    size: usize,
}

impl<ALG: tensor::TensorAlg> Universe<ALG> {
    pub fn new(alg: ALG, size: usize) -> Self {
        Self { alg, size }
    }
}

impl<ALG> Universe<ALG>
where
    ALG: tensor::TensorAlg,
{
    pub fn is_scalar(self: &Self, elem: &ALG::Elem) -> bool {
        ALG::shape(elem).is_empty()
    }

    pub fn is_relation(self: &Self, elem: &ALG::Elem) -> bool {
        ALG::shape(elem).is_rectangular(self.size)
    }

    pub fn is_binary_rel(self: &Self, elem: &ALG::Elem) -> bool {
        ALG::shape(elem).len() == 2 && self.is_relation(elem)
    }

    fn new_shape(self: &Self, len: usize) -> tensor::Shape {
        let mut shape = Vec::with_capacity(len);
        shape.resize(len, self.size);
        tensor::Shape::new(shape)
    }
}

// TODO: move this to tensors
impl<ALG> boolean::BoolAlg for Universe<ALG>
where
    ALG: tensor::TensorAlg,
{
    type Elem = ALG::Elem;

    fn bool_lift(self: &Self, elem: bool) -> Self::Elem {
        self.alg
            .tensor_lift(tensor::Tensor::create(self.new_shape(0), |_| elem))
    }

    fn bool_not(self: &mut Self, elem: Self::Elem) -> Self::Elem {
        assert!(self.is_scalar(&elem));
        self.alg.tensor_not(elem)
    }

    fn bool_or(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert!(self.is_scalar(&elem1));
        assert!(self.is_scalar(&elem2));
        self.alg.tensor_or(elem1, elem2)
    }

    fn bool_xor(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert!(self.is_scalar(&elem1));
        assert!(self.is_scalar(&elem2));
        self.alg.tensor_xor(elem1, elem2)
    }

    fn bool_and(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert!(self.is_scalar(&elem1));
        assert!(self.is_scalar(&elem2));
        self.alg.tensor_and(elem1, elem2)
    }

    fn bool_equ(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert!(self.is_scalar(&elem1));
        assert!(self.is_scalar(&elem2));
        self.alg.tensor_equ(elem1, elem2)
    }

    fn bool_imp(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert!(self.is_scalar(&elem1));
        assert!(self.is_scalar(&elem2));
        self.alg.tensor_imp(elem1, elem2)
    }
}

pub trait BinaryRelAlg {
    type Elem: Clone;

    /// Returns the empty or total relation.
    fn binrel_lift(self: &mut Self, elem: bool) -> Self::Elem;

    /// Returns the empty relation.
    fn binrel_empty(self: &mut Self) -> Self::Elem {
        self.binrel_lift(false)
    }

    /// Returns the total relation.
    fn binrel_total(self: &mut Self) -> Self::Elem {
        self.binrel_lift(true)
    }

    /// Returns the diagonal relation.
    fn binrel_diag(self: &mut Self) -> Self::Elem;

    /// Returns the complement of the given relation.
    fn binrel_comp(self: &mut Self, elem: Self::Elem) -> Self::Elem;

    /// Intersection of a pair of relations.
    fn binrel_meet(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;

    /// Union of a pair of relations.
    fn binrel_join(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        let elem1 = self.binrel_comp(elem1);
        let elem2 = self.binrel_comp(elem2);
        let elem3 = self.binrel_meet(elem1, elem2);
        self.binrel_comp(elem3)
    }

    /// Returns the inverse of the relation.
    fn binrel_inv(self: &mut Self, elem: Self::Elem) -> Self::Elem;

    /// Returns the composition of a pair of relations.
    fn binrel_circ(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem;
}

impl<ALG> BinaryRelAlg for Universe<ALG>
where
    ALG: tensor::TensorAlg,
{
    type Elem = ALG::Elem;

    fn binrel_lift(self: &mut Self, elem: bool) -> Self::Elem {
        self.alg
            .tensor_lift(tensor::Tensor::create(self.new_shape(2), |_| elem))
    }

    fn binrel_diag(self: &mut Self) -> Self::Elem {
        self.alg
            .tensor_lift(tensor::Tensor::create(self.new_shape(2), |c| c[0] == c[1]))
    }

    fn binrel_comp(self: &mut Self, elem: Self::Elem) -> Self::Elem {
        assert!(self.is_binary_rel(&elem));
        self.alg.tensor_not(elem)
    }

    fn binrel_meet(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert!(self.is_binary_rel(&elem1));
        self.alg.tensor_and(elem1, elem2)
    }

    fn binrel_join(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert!(self.is_binary_rel(&elem1));
        self.alg.tensor_or(elem1, elem2)
    }

    fn binrel_inv(self: &mut Self, elem: Self::Elem) -> Self::Elem {
        assert!(self.is_binary_rel(&elem));
        self.alg.tensor_polymer(elem, self.new_shape(2), &[1, 0])
    }

    fn binrel_circ(self: &mut Self, elem1: Self::Elem, elem2: Self::Elem) -> Self::Elem {
        assert!(self.is_binary_rel(&elem1));
        assert!(self.is_binary_rel(&elem2));
        let elem1 = self.alg.tensor_polymer(elem1, self.new_shape(3), &[1, 0]);
        let elem2 = self.alg.tensor_polymer(elem2, self.new_shape(3), &[0, 2]);
        let elem3 = self.alg.tensor_and(elem1, elem2);
        self.alg.tensor_any(elem3)
    }
}
