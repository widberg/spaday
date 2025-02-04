use binrw::{BinReaderExt, BinResult};
use clap::Parser;
use exe::{FlattenedResourceDataEntry, ResolvedDirectoryID, ResourceDirectory, VecPE};
use flate2::read::GzDecoder;
use serde::Serialize;
use std::{
    io::{Cursor, Read, SeekFrom},
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    name: PathBuf,
    #[clap(short, long)]
    out: Option<PathBuf>,
}

#[binrw::binread]
#[derive(Debug)]
struct XdbfFreeSpaceEntry {
    _offset_specifier: u32,
    _length: u32,
}

#[binrw::binread]
#[derive(Debug)]
struct XdbfSectionHeader {
    _version: u32,
    _size: u32,
}

#[binrw::binread]
#[derive(Debug, Serialize)]
struct XachAchievement {
    achievement_id: u16,
    title_string_id: u16,
    unlocked_description_id: u16,
    locked_description_id: u16,
    image_id: u32,
    #[br(pad_after = 2)]
    gamerscore: u16,
    #[br(pad_after = 16)]
    flags: u32,
}

#[binrw::binread]
#[br(magic = 0x58414348u32)]
#[derive(Debug, Serialize)]
struct Xach {
    #[br(temp)]
    _header: XdbfSectionHeader,
    #[br(temp)]
    achievement_count: u16,
    #[br(count = achievement_count)]
    achievements: Vec<XachAchievement>,
}

#[binrw::binread]
#[derive(Debug, Serialize)]
struct XcxtRecord {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
}

#[binrw::binread]
#[br(magic = 0x58435854u32)]
#[derive(Debug, Serialize)]
struct Xcxt {
    #[br(temp)]
    _header: XdbfSectionHeader,
    #[br(temp)]
    record_count: u32,
    #[br(count = record_count)]
    records: Vec<XcxtRecord>,
}

#[binrw::binread]
#[derive(Debug, Serialize)]
struct XitbImage {
    image_id: u32,
    #[br(temp)]
    name_length: u32,
    #[br(try_map = |x: Vec<u8>| String::from_utf8(x), count = name_length)]
    image_path: String,
}

#[binrw::binread]
#[br(magic = 0x58495442u32)]
#[derive(Debug, Serialize)]
struct Xitb {
    #[br(temp)]
    _header: XdbfSectionHeader,
    #[br(temp)]
    image_count: u32,
    #[br(count = image_count)]
    images: Vec<XitbImage>,
}

#[binrw::binread]
#[br(magic = 0x584D4154u32)]
#[derive(Debug, Serialize)]
struct Xmat {
    #[br(temp)]
    _header: XdbfSectionHeader,
    property_bag_metadata: Xpbm,
}

#[binrw::binread]
#[br(magic = 0x5850424Du32)]
#[derive(Debug, Serialize)]
struct Xpbm {
    #[br(temp)]
    _header: XdbfSectionHeader,
    context_count: u32,
    property_count: u32,
    #[br(count = context_count)]
    contexts: Vec<u32>,
    #[br(count = property_count)]
    properties: Vec<u32>,
}

#[binrw::binread]
#[derive(Debug, Serialize)]
struct XprpRecord {
    unknown0: u64,
}

#[binrw::binread]
#[br(magic = 0x58505250u32)]
#[derive(Debug, Serialize)]
struct Xprp {
    #[br(temp)]
    _header: XdbfSectionHeader,
    #[br(temp)]
    record_count: u16,
    #[br(count = record_count)]
    records: Vec<XprpRecord>,
}

#[binrw::binread]
#[br(magic = 0x58525054u32)]
#[derive(Debug, Serialize)]
struct Xrpt {
    #[br(temp)]
    _header: XdbfSectionHeader,
    property_bag_metadata: Xpbm,
    #[br(temp)]
    property_bag_metadata_count: u16,
    #[br(count = property_bag_metadata_count)]
    property_bag_metadatas: Vec<Xpbm>,
}

#[binrw::parser(reader)]
fn parse_gzip() -> BinResult<Vec<u8>> {
    let mut x = Vec::<u8>::new();
    GzDecoder::new(reader).read_to_end(&mut x)?;
    Ok(x)
}

#[binrw::binread]
#[br(magic = 0x58535243u32)]
#[derive(Debug, Serialize)]
struct Xsrc {
    #[br(temp)]
    _header: XdbfSectionHeader,
    #[br(temp)]
    name_length: u32,
    #[br(try_map = |x: Vec<u8>| String::from_utf8(x), count = name_length)]
    filename: String,
    #[br(temp)]
    _gzip_file_uncompressed_size: u32,
    #[br(temp)]
    _gzip_file_compressed_size: u32,
    #[br(parse_with = parse_gzip)]
    gzip_file: Vec<u8>,
}

#[binrw::binread]
#[br(magic = 0x58535443u32)]
#[derive(Debug, Serialize)]
struct Xstc {
    #[br(temp)]
    _header: XdbfSectionHeader,
    unknown0: u32,
}

#[binrw::binread]
#[br(repr = u32)]
#[derive(Debug, Serialize)]
enum TitleType {
    System = 0,
    Full = 1,
    Demo = 2,
    Download = 3,
}

#[binrw::binread]
#[br(magic = 0x58544844u32)]
#[derive(Debug, Serialize)]
struct Xthd {
    #[br(temp)]
    _header: XdbfSectionHeader,
    title_id: u32,
    title_type: TitleType,
    project_version_major: u16,
    project_version_minor: u16,
    project_version_build: u16,
    #[br(pad_after = 16)]
    project_version_revision: u16,
}

#[binrw::binread]
#[derive(Debug, Serialize)]
struct Xvc2ViewFieldEntry {
    size: u32,
    property_id: u32,
    flags: u32,
    attribute_id: u16,
    string_id: u16,
    aggregation_type: u16,
    ordinal: u8,
    field_type: u8,
    #[br(pad_after = 8)]
    format_type: u32,
}

#[binrw::binread]
#[derive(Debug, Serialize)]
struct Xvc2SharedView {
    column_count: u16,
    #[br(pad_after = 8)]
    row_count: u16,
    #[br(count = column_count)]
    column_entries: Vec<Xvc2ViewFieldEntry>,
    #[br(count = row_count)]
    row_entries: Vec<Xvc2ViewFieldEntry>,
    property_bag_metadata: Xpbm,
}

#[binrw::binread]
#[derive(Debug, Serialize)]
struct Xvc2StatsViewTableEntry {
    id: u32,
    flags: u32,
    shared_index: u16,
    #[br(pad_after = 4)]
    string_id: u16,
}

#[binrw::binread]
#[br(magic = 0x58564332u32)]
#[derive(Debug, Serialize)]
struct Xvc2 {
    #[br(temp)]
    _header: XdbfSectionHeader,
    shared_view_count: u16,
    #[br(count = shared_view_count)]
    shared_views: Vec<Xvc2SharedView>,
    view_count: u16,
    #[br(count = view_count)]
    views: Vec<Xvc2StatsViewTableEntry>,
}

#[binrw::binread]
#[derive(Debug, Serialize)]
struct XstrString {
    string_id: u16,
    #[br(temp)]
    string_length: u16,
    #[br(try_map = |x: Vec<u8>| String::from_utf8(x), count = string_length)]
    string: String,
}

#[binrw::binread]
#[br(magic = 0x58535452u32)]
#[derive(Debug, Serialize)]
struct Xstr {
    #[br(temp)]
    _header: XdbfSectionHeader,
    #[br(temp)]
    string_count: u16,
    #[br(count = string_count)]
    strings: Vec<XstrString>,
}

#[binrw::binread]
#[derive(Debug, Serialize)]
#[serde(untagged)]
enum XdbfEntryStructuredData {
    Xach(Xach),
    Xcxt(Xcxt),
    Xitb(Xitb),
    Xmat(Xmat),
    Xpbm(Xpbm),
    Xprp(Xprp),
    Xrpt(Xrpt),
    Xsrc(Xsrc),
    Xstc(Xstc),
    Xthd(Xthd),
    Xvc2(Xvc2),
    Xstr(Xstr),
}

#[binrw::binread]
#[br(import(length: u32))]
#[derive(Debug)]
enum XdbfEntryData {
    XdbfEntryStructuredData(XdbfEntryStructuredData),
    Bin(#[br(count = length)] Vec<u8>),
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
        args(_length), restore_position)]
    data: XdbfEntryData,
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
        match (entry.namespace, &entry.data) {
            (_, XdbfEntryData::XdbfEntryStructuredData(_)) => "json",
            (2, _) => "png",
            (_, _) => "bin",
        }
    ))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let data = std::fs::read(&args.name)?;

    let xdbf: Xdbf = match Cursor::new(&data).read_be() {
        Ok(xdbf) => xdbf,
        Err(_) => {
            let pe = VecPE::from_disk_data(&data);
            let resource_directory = ResourceDirectory::parse(&pe)?;
            let spafile_resource = resource_directory
                .resources
                .iter()
                .find(|r| {
                    matches!(r,
                        FlattenedResourceDataEntry {
                            rsrc_id: ResolvedDirectoryID::Name(r),
                            type_id: ResolvedDirectoryID::Name(t),
                            ..
                        } if r == "SPAFILE" && t == "RT_RCDATA"
                    )
                })
                .ok_or("Could not find SPAFILE resource in executable.")?;
            let spafile_data = spafile_resource.get_data_entry(&pe)?.read(&pe)?;
            Cursor::new(spafile_data).read_be()?
        }
    };

    let dir: &Path = args
        .out
        .as_deref()
        .or_else(|| args.name.parent())
        .unwrap_or_else(|| Path::new("."));

    std::fs::create_dir_all(dir)?;

    for entry in xdbf.entry_table {
        let path = id_to_pathbuf(&entry);
        println!("{:?}", path);

        let output_path = dir.join(path);
        match &entry.data {
            XdbfEntryData::XdbfEntryStructuredData(data) => {
                let json = serde_json::to_string_pretty(&data)?;
                std::fs::write(&output_path, json)?;
            }
            XdbfEntryData::Bin(data) => std::fs::write(&output_path, data)?,
        }

        if let XdbfEntryData::XdbfEntryStructuredData(XdbfEntryStructuredData::Xsrc(xsrc)) =
            entry.data
        {
            println!("{:?}", xsrc.filename);
            let output_path = dir.join(xsrc.filename);
            std::fs::write(output_path, xsrc.gzip_file)?;
        }
    }

    Ok(())
}
