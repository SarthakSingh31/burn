use crate::tensor::{
    backend::tch::{TchShape, TchTensor},
    ops::*,
};
use std::ops::Range;

impl<P: tch::kind::Element + std::fmt::Debug + Copy + Default, const D1: usize>
    TensorOpsIndex<P, D1> for TchTensor<P, D1>
{
    fn index<const D2: usize>(&self, indexes: [Range<usize>; D2]) -> Self {
        let mut tensor = self.tensor.shallow_clone();

        for (i, index) in indexes.iter().enumerate().take(D2) {
            let start = index.start as i64;
            let length = (index.end - index.start) as i64;
            tensor = tensor.narrow(i as i64, start, length);
        }

        let shape = self.shape.index(indexes);
        let kind = self.kind;

        Self {
            kind,
            tensor,
            shape,
        }
    }

    fn index_assign<const D2: usize>(&self, indexes: [Range<usize>; D2], values: &Self) -> Self {
        let tensor_original = self.tensor.copy();
        let tch_shape = TchShape::from(self.shape);

        let mut tensor = tensor_original.view_(&tch_shape.dims);

        for (i, index) in indexes.into_iter().enumerate().take(D2) {
            let start = index.start as i64;
            let length = (index.end - index.start) as i64;

            tensor = tensor.narrow(i as i64, start, length);
        }

        tensor.copy_(&values.tensor);

        let shape = self.shape;
        let kind = self.kind;

        Self {
            kind,
            tensor: tensor_original,
            shape,
        }
    }
}
