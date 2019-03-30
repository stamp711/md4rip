pub use block_buffer::generic_array::{
    typenum::{U16, U64},
    GenericArray,
};

pub use block_buffer::BlockBuffer;

pub type U8Block = GenericArray<u8, U64>;
pub type U32Block = GenericArray<u32, U16>;
