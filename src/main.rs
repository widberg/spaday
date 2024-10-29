use binrw::{io::BufReader, BinRead, BinReaderExt};
use clap::Parser;
use std::{fs::File, io::SeekFrom, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    name: PathBuf,
}

#[binrw::binread]
#[derive(Debug)]
struct XdbfFreeSpaceEntry {
    _offset_specifier: u32,
    _length: u32,
}

#[derive(Debug)]
enum XdbfId {
    String(String),
    Number(u64),
}

impl BinRead for XdbfId {
    type Args<'a> = ();
    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes)?;
        let first_non_zero_index = bytes.iter().position(|&b| b != 0).unwrap_or(8);
        let trimmed = &bytes[first_non_zero_index..];
        if let Ok(string) = std::str::from_utf8(trimmed) {
            if !string.is_empty() && string.chars().all(|c| c.is_ascii_graphic()) {
                return Ok(Self::String(string.to_owned()));
            }
        }

        let number = u64::from_be_bytes(bytes);
        Ok(Self::Number(number))
    }
}

#[binrw::binread]
#[br(import(data_entry_offset: u32))]
#[derive(Debug)]
struct XdbfEntry {
    namespace: u16,
    id: XdbfId,
    _offset_specifier: u32,
    _length: u32,
    #[br(seek_before = SeekFrom::Start((data_entry_offset + _offset_specifier).into()),
        count = _length, restore_position)]
    data: Vec<u8>,
}

#[binrw::binread]
#[derive(Debug)]
struct XdbfHeader {
    _magic: u32,
    _version: u32,
    entry_table_length: u32,
    entry_count: u32,
    free_space_table_length: u32,
    free_space_table_entry_count: u32,
}

#[binrw::binread]
#[derive(Debug)]
struct Xdbf {
    _header: XdbfHeader,
    #[br(count = _header.entry_count,
        pad_after = (_header.entry_table_length - _header.entry_count) * 18,
        args { inner: (_header.entry_table_length * 18 + _header.free_space_table_length * 8 + 24,) })]
    entry_table: Vec<XdbfEntry>,
    #[br(count = _header.free_space_table_entry_count,
        pad_after = (_header.free_space_table_length - _header.free_space_table_entry_count) * 8)]
    _free_space_table: Vec<XdbfFreeSpaceEntry>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let reader = File::open(&args.name)?;
    let mut bufreader = BufReader::new(reader);

    let xdbf: Xdbf = bufreader.read_be()?;

    let dir = args.name.parent().unwrap();

    for entry in xdbf.entry_table {
        println!("{:?} {:?}", entry.namespace, entry.id);

        let output_path = dir.join(PathBuf::from(format!(
            "{}.{}.{}",
            entry.namespace,
            match entry.id {
                XdbfId::String(string) => string,
                XdbfId::Number(number) => number.to_string(),
            },
            if entry.namespace == 2 { "png" } else { "bin" }
        )));
        std::fs::write(output_path, entry.data)?;
    }

    Ok(())
}
