mod buf;
mod buf_mut;

pub use buf::OraBufExt;
pub use buf_mut::OraBufMutExt;

pub(crate) use sqlx_core::io::*;

#[derive(Debug)]
pub(crate) struct RowId {
    pub(crate) rba: u32,
    pub(crate) partition_id: u16,
    pub(crate) block_num: u32,
    pub(crate) slot_num: u16,
}

impl RowId {
    fn new(rba: u32, partition_id: u16, block_num: u32, slot_num: u16) -> Self {
        Self {
            rba,
            partition_id,
            block_num,
            slot_num,
        }
    }
}
