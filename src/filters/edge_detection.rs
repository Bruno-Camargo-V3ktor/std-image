use super::{Filter, FilterError};

pub struct EdgeDetection;

//# # #    //# # #   //# # #
const KERNEL_TABLE: [i8; 9] = [-1, 0, 1, -2, 0, 2, -1, 0, 1];

impl Filter for EdgeDetection {
    fn apply(&self, image: &mut impl crate::images::Image) -> Result<(), FilterError> {
        Ok(())
    }
}
