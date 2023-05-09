use binrw::prelude::*;
use binrw::{BinRead, PosValue};
use std::fmt::{Debug, Formatter};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

#[binrw]
#[brw(little, magic = b"SARC")]
#[derive(Debug)]
pub struct SarcArchive {
    pub version: u32,
    #[br(temp)]
    #[bw(calc = files.len() as u32)]
    pub count: u32,
    #[br(count = count)]
    pub files: Vec<FilePointer>,
    #[bw(ignore)]
    pub position: PosValue<()>,
}

#[binrw]
#[derive(Debug)]
pub struct FilePointer {
    #[br(temp)]
    #[bw(calc = path.len() as u8)]
    pub path_len: u8,
    #[br(count = path_len, map = |v| String::from_utf8(v).unwrap(), pad_after = 1)]
    #[bw(map = |s| s.as_bytes())]
    pub path: String,
    pub position: u32,
    pub size: u32,
    #[br(assert(size == size_2), temp)]
    #[bw(calc = *size)]
    pub size_2: u32,
}

impl std::fmt::Display for SarcArchive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "v{}", self.version)?;
        for file in self.files.iter() {
            writeln!(f, "{}", file)?;
        }
        writeln!(f, "=> {:#x}", self.position.pos)
    }
}

impl std::fmt::Display for FilePointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} => {:#x}..+{:#x}",
            self.path, self.position, self.size
        )
    }
}

impl SarcArchive {
    pub fn read_file(path: &str) -> std::io::Result<SarcArchive> {
        let mut file = File::open(path)?;
        SarcArchive::read(&mut file)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
    }

    pub fn read<R>(file: &mut R) -> BinResult<SarcArchive>
    where
        R: Read + Seek,
    {
        BinRead::read(file)
    }

    pub fn extract<R>(&self, file: &mut R, path: &str) -> std::io::Result<Vec<u8>>
    where
        R: Read + Seek,
    {
        self.files
            .iter()
            .find(|it| it.path.as_str() == path)
            .ok_or(std::io::Error::new(std::io::ErrorKind::NotFound, path))
            .and_then(|ptr| ptr.extract(file, self.position.pos))
    }

    pub fn extract_all(path: &str) -> std::io::Result<()> {
        let info = SarcArchive::read_file(path)?;
        let mut file = File::open(path)?;
        let dir = Path::new(path);

        for ptr in info.files {
            let out_path = dir.with_file_name(format!("extract\\{}", ptr.path.replace(':', "")));

            println!("Extracting: {}", ptr.path);
            println!("            ↳ {}", out_path.to_str().unwrap());

            fs::create_dir_all(out_path.with_file_name(""))?;

            let mut data = ptr
                .extract(&mut file, info.position.pos)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
            let mut output = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(out_path)?;
            output.write_all(data.as_mut_slice())?;
        }

        Ok(())
    }
}

impl FilePointer {
    fn extract<R>(&self, file: &mut R, offset: u64) -> std::io::Result<Vec<u8>>
    where
        R: Read + Seek,
    {
        file.seek(SeekFrom::Start(self.position as u64 + offset))?;
        let mut data = vec![0u8; self.size as usize];
        file.read_exact(data.as_mut_slice())?;
        Ok(data)
    }
}
