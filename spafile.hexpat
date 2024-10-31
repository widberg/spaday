import std.core;

struct XdbfSectionHeader {
    u32 magic;
    u32 version;
    u32 size;
};

struct XachAchievement {
    u16 achievement_id;
    u16 title_string_id;
    u16 unlocked_description_id;
    u16 locked_description_id;
    u32 image_id;
    u16 gamerscore;
    padding[2];
    u32 flags;
    padding[16];
};

struct Xach {
    XdbfSectionHeader header;
    u16 achievement_count;
    XachAchievement achievements[achievement_count];
};

struct XcxtRecord {
    u32;
    u32;
    u32;
    u32;
};

struct Xcxt {
    XdbfSectionHeader header;
    u32 record_count;
    XcxtRecord records[record_count];
};

struct XitbImage {
    u32 image_id;
    u32 name_length;
    char image_path[name_length];
};

struct Xitb {
    XdbfSectionHeader header;
    u32 image_count;
    XitbImage images[image_count];
};

struct Xpbm {
    XdbfSectionHeader header;
    u32 context_count;
    u32 property_count;
    u32 contexts[context_count];
    u32 properties[property_count];
};

struct Xmat {
    XdbfSectionHeader header;
    Xpbm property_bag_metadata;
};

struct XprpRecord {
    u64;
};

struct Xprp {
    XdbfSectionHeader header;
    u16 record_count;
    XprpRecord records[record_count];
};

struct Xrpt {
    XdbfSectionHeader header;
    Xpbm property_bag_metadata;
    u16 property_bag_metadata_count;
    Xpbm property_bag_metadatas[property_bag_metadata_count];
};

struct Xsrc {
    XdbfSectionHeader header;
    u32 name_length;
    char filename[name_length];
    u32 gzip_file_uncompressed_size;
    u32 gzip_file_compressed_size;
    u8 gzip_file[gzip_file_compressed_size];
};

struct Xstc {
    XdbfSectionHeader header;
    u32;
};

enum TitleType : u32 {
    System = 0,
    Full = 1,
    Demo = 2,
    Download = 3,
};

struct Xthd {
    XdbfSectionHeader header;
    u32 title_id;
    TitleType title_type;
    u16 project_version_major;
    u16 project_version_minor;
    u16 project_version_build;
    u16 project_version_revision;
    padding[16];
};

struct Xvc2ViewFieldEntry {
    u32 size;
    u32 property_id;
    u32 flags;
    u16 attribute_id;
    u16 string_id;
    u16 aggregation_type;
    u8 ordinal;
    u8 field_type;
    u32 format_type;
    padding[8];
};

struct Xvc2SharedView {
    u16 column_count;
    u16 row_count;
    padding[8];
    Xvc2ViewFieldEntry column_entries[column_count];
    Xvc2ViewFieldEntry row_entries[row_count];
    Xpbm property_bag_metadata;
};

struct Xvc2StatsViewTableEntry {
    u32 id;
    u32 flags;
    u16 shared_index;
    u16 string_id;
    padding[4];
};

struct Xvc2 {
    XdbfSectionHeader header;
    u16 shared_view_count;
    Xvc2SharedView shared_views[shared_view_count];
    u16 view_count;
    Xvc2StatsViewTableEntry views[view_count];
};

struct XstrString {
    u16 string_id;
    u16 string_length;
    char string[string_length];
};

struct Xstr {
    XdbfSectionHeader header;
    u16 string_count;
    XstrString strings[string_count];
};

struct XdbfFreeSpaceEntry {
    u32 offset_specifier;
    u32 length;
};

struct XdbfEntry {
    u16 ns;
    u64 id;
    u32 offset_specifier;
    u32 length;
    u32 offset = offset_specifier + 24 + parent.header.entry_table_length * 18 + parent.header.free_space_table_length * 8;
    match (ns, id) {
        (1, 1480672072): Xach xach @ offset;
        (1, 1480808532): Xcxt xcxt @ offset;
        (1, 1481200706): Xitb xitb @ offset;
        (1, 1481458004): Xmat xmat @ offset;
        (1, 1481654861): Xpbm xpbm @ offset;
        (1, 1481658960): Xprp xprp @ offset;
        (1, 1481789524): Xrpt xrpt @ offset;
        (1, 1481855555): Xsrc xsrc @ offset;
        (1, 1481856067): Xstc xstc @ offset;
        (1, 1481918532): Xthd xthd @ offset;
        (1, 1482048306): Xvc2 xvc2 @ offset;
        (3, _): Xstr xstr @ offset;
        (_,_): u8 data[length] @ offset;
    }
};

struct XdbfHeader {
    u32 version;
    u32 entry_table_length;
    u32 entry_count;
    u32 free_space_table_length;
    u32 free_space_table_entry_count;
};

struct Xdbf {
    be u32 magic;
    if (magic == 1480868422) {
        std::core::set_endian(std::mem::Endian::Big);
    } else {
        std::core::set_endian(std::mem::Endian::Little);
    }
    XdbfHeader header;
    XdbfEntry entry_table[header.entry_count];
    XdbfEntry entry_table_trail[header.entry_table_length - header.entry_count] [[hidden]];
    XdbfFreeSpaceEntry free_space_table[header.free_space_table_entry_count];
    XdbfFreeSpaceEntry free_space_table_trail[header.free_space_table_length - header.free_space_table_entry_count] [[hidden]];
};

Xdbf xdbf @ 0x0;
