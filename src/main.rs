use binrw::{io::BufReader, BinReaderExt};
use clap::Parser;
use std::{
    fs::File,
    io::SeekFrom,
    path::{Path, PathBuf},
};

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

#[binrw::binread]
#[br(import(data_entry_offset: u32))]
#[derive(Debug)]
struct XdbfEntry {
    namespace: u16,
    id: u64,
    _offset_specifier: u32,
    _length: u32,
    #[br(seek_before = SeekFrom::Start((data_entry_offset + _offset_specifier).into()),
        count = _length, restore_position)]
    data: Vec<u8>,
}

#[binrw::binread]
#[derive(Debug)]
struct XdbfHeader {
    _version: u32,
    entry_table_length: u32,
    entry_count: u32,
    free_space_table_length: u32,
    free_space_table_entry_count: u32,
}

#[binrw::binread]
#[derive(Debug)]
#[br(stream = s, is_big = s.read_be::<u32>()? == 0x58444246)]
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

fn id_to_pathbuf(entry: &XdbfEntry) -> PathBuf {
    PathBuf::from(format!(
        "{}.{}.{}",
        entry.namespace,
        {
            let bytes: [u8; 8] = u64::to_be_bytes(entry.id);
            let result = match entry.namespace {
                1 => {
                    let first_non_zero_index = bytes.iter().position(|&b| b != 0).unwrap_or(8);
                    let trimmed = &bytes[first_non_zero_index..];

                    std::str::from_utf8(trimmed).ok().filter(|string| {
                        !string.is_empty() && string.chars().all(|c| c.is_ascii_graphic())
                    })
                }
                3 => match entry.id {
                    1 => Some("en-US"),
                    2 => Some("ja-JP"),
                    3 => Some("de-DE"),
                    4 => Some("fr-FR"),
                    5 => Some("es-ES"),
                    6 => Some("it-IT"),
                    7 => Some("ko-KR"),
                    8 => Some("zh-CHT"),
                    9 => Some("pt-PT"),
                    10 => Some("zh-CHS"),
                    11 => Some("pl-PL"),
                    12 => Some("ru-RU"),
                    _ => None,
                },
                _ => None,
            };

            match result {
                Some(s) => s.to_string(),
                None => entry.id.to_string(),
            }
        },
        if entry.namespace == 2 { "png" } else { "bin" }
    ))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let reader = File::open(&args.name)?;
    let mut bufreader = BufReader::new(reader);

    let xdbf: Xdbf = bufreader.read_be()?;

    let dir = args.name.parent().unwrap_or_else(|| Path::new("."));

    for entry in xdbf.entry_table {
        let path = id_to_pathbuf(&entry);
        println!("{:?}", path);

        let output_path = dir.join(path);
        std::fs::write(output_path, entry.data)?;
    }

    Ok(())
}
