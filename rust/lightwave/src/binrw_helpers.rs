use crate::lwo2::vx;
use binrw::{binread, BinRead, BinResult, Endian};
use std::io::{Read, Seek};
use std::iter::from_fn;

#[binread]
#[br(assert(false, "Not implemented yet"))]
#[derive(Debug)]
pub struct BinReadTodo();

pub fn until_size_limit<R, Arg, T, Ret>(
    limit: u64,
) -> impl Fn(&mut R, Endian, Arg) -> BinResult<Ret>
where
    T: for<'a> BinRead<Args<'a> = Arg>,
    R: Read + Seek,
    Arg: Clone,
    Ret: FromIterator<T>,
{
    until_size_limit_with(limit, default_reader)
}

/// Reads data until total size reaches a limit
pub fn until_size_limit_with<R, Arg, T, ReadFn, Ret>(
    limit: u64,
    reader_fn: ReadFn,
) -> impl Fn(&mut R, Endian, Arg) -> BinResult<Ret>
where
    T: for<'a> BinRead<Args<'a> = Arg>,
    R: Read + Seek,
    Arg: Clone,
    ReadFn: Fn(&mut R, Endian, Arg) -> BinResult<T>,
    Ret: FromIterator<T>,
{
    move |reader, endian, args| {
        let pos = reader.stream_position()?;
        from_fn(|| match reader.stream_position() {
            Ok(now) if now - pos < limit => Some(reader_fn(reader, endian, args.clone())),
            Ok(_) => None,
            Err(err) => Some(Err(binrw::Error::Io(err))),
        })
        .fuse()
        .collect()
    }
}

pub fn count_with_vx<R>(n: usize) -> impl Fn(&mut R, Endian, ()) -> BinResult<Vec<u32>>
where
    R: Read + Seek,
{
    move |reader, endian, _args: ()| {
        core::iter::repeat_with(|| vx(reader, endian, ()))
            .take(n)
            .collect()
    }
}

fn default_reader<'a, T: BinRead, R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    args: T::Args<'a>,
) -> BinResult<T>
where
    T::Args<'a>: Clone,
{
    let mut value = T::read_options(reader, endian, args.clone())?;
    value.after_parse(reader, endian, args)?;
    Ok(value)
}
