use binrw::{binread, BinRead};
use std::ops::Deref;

#[binread]
#[derive(Debug)]
pub struct Chunk<D>
where
    for<'a> D: BinRead<Args<'a> = (u32,)>,
{
    pub length: u32,
    #[br(pad_size_to = length, align_after = 2, args(length))]
    pub data: D,
}

impl<D> Deref for Chunk<D>
where
    for<'a> D: BinRead<Args<'a> = (u32,)>,
{
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[binread]
#[derive(Debug)]
pub struct SubChunk<D>
where
    for<'a> D: BinRead<Args<'a> = (u32,)>,
{
    pub length: u16,
    #[br(pad_size_to = length, align_after = 2, args(length as u32))]
    pub data: D,
}

impl<D> Deref for SubChunk<D>
where
    for<'a> D: BinRead<Args<'a> = (u32,)>,
{
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
