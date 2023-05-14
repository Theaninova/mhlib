use binrw::io::{TakeSeek, TakeSeekExt};
use binrw::meta::{EndianKind, ReadEndian};
use binrw::{until_eof, BinRead, BinReaderExt, BinResult, Endian};
use std::io::{Read, Seek, SeekFrom};

pub struct Chunk<T, Args = ()>(pub T)
where
    for<'a> T: BinRead<Args<'a> = Args>;

impl<T, Args> Chunk<T, Args>
where
    for<'a> T: BinRead<Args<'a> = Args>,
{
    pub fn inner(self) -> T {
        self.0
    }
}

pub fn chunk_until_eof<Reader, T, Arg, Ret>(
    reader: &mut Reader,
    endian: Endian,
    args: Arg,
) -> BinResult<Ret>
where
    T: for<'a> BinRead<Args<'a> = Arg>,
    Reader: Read + Seek,
    Arg: Clone,
    Ret: FromIterator<T>,
{
    let len = reader.read_type::<u32>(endian)? as u64 - 6;
    let pos = reader.stream_position()?;
    let end = pos + len;
    let mut sub_stream = reader.take_seek(len);
    let result = until_eof(&mut sub_stream, endian, args)?;
    reader.seek(SeekFrom::Start(end))?;
    Ok(result)
}

impl<T, Args> ReadEndian for Chunk<T, Args>
where
    for<'a> T: BinRead<Args<'a> = Args>,
{
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

impl<T, Args> BinRead for Chunk<T, Args>
where
    for<'a> T: BinRead<Args<'a> = Args>,
{
    type Args<'a> = Args;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let len = reader.read_type::<u32>(endian)? as u64 - 6;
        let pos = reader.stream_position()?;
        let end = pos + len;
        let mut sub_stream = reader.take_seek(len);
        let result = T::read_options(&mut sub_stream, endian, args)?;
        reader.seek(SeekFrom::Start(end))?;
        Ok(Chunk(result))
    }
}
