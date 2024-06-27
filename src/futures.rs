use std::num::NonZeroU32;

pub trait U32Future {
    fn update_value(&self, val: NonZeroU32);
}
