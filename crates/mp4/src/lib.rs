pub mod misc;

#[derive(Debug, Clone)]
pub struct MpegBox {
    pub size: u64,
    pub name: String,
    pub usertype: Option<[u8; 16]>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct MpegFullBox {
    pub size: u64,
    pub name: String,
    /// only usertype box has this field
    pub usertype: Option<[u8; 16]>,
    pub version: u8,
    pub flags: [u8; 3],
    pub data: Vec<u8>,
}

impl From<MpegBox> for MpegFullBox {
    fn from(value: MpegBox) -> Self {
        Self {
            size: value.size,
            name: value.name,
            usertype: value.usertype,
            version: value.data[0],
            flags: value.data[1..4].try_into().unwrap(),
            data: value.data[4..].into(),
        }
    }
}

pub fn read_box(input: &mut dyn std::io::Read) -> Result<MpegBox, Box<dyn std::error::Error>> {
    // TODO: partial read
    let mut buf = [0u8; 4];
    input.read_exact(&mut buf)?;
    let mut size = misc::parse_u32(buf.as_slice()) as u64;  // big endian
    input.read_exact(&mut buf)?;
    let name = misc::parse_utf8(buf.as_slice());
    let mut data = vec!();
    let mut used = 8;
    if size == 1 {
        // largesize
        let mut largebuf = [0u8; 8];
        input.read_exact(&mut largebuf)?;
        size = u64::from_be_bytes(largebuf);
        used += 8;
    }
    let mut usertype = None;
    if name == "uuid" {
        let mut buf = [0u8; 16];
        input.read_exact(&mut buf)?;
        usertype = Some(buf);
        used += 16;
    }
    if size == 0 {
        // read till EOF
        input.read_to_end(&mut data)?;
        size = used + data.len() as u64;
    } else {
        data.resize((size - used) as usize, 0u8);
        input.read_exact(&mut data)?;
    }

    Ok( MpegBox { size, name, usertype, data } )
}

#[derive(Debug, Clone)]
pub struct FileTypeBox {
    pub major_brand: String,
    pub minor_version: u32,
    pub compatible_brands: Vec<String>,
}

impl From<MpegBox> for FileTypeBox {
    fn from(value: MpegBox) -> Self {
        Self {
            major_brand: misc::parse_utf8(&value.data[0..4]),
            minor_version: misc::parse_u32(&value.data[4..8]),
            compatible_brands: value.data[8..].chunks(4).map(misc::parse_utf8).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MovieHeaderBox {
    pub flags: [u8; 3],
    pub version: u8,
    pub creation_time: u64,
    pub modification_time: u64,
    pub timescale: u32,
    pub duration: u64,
    /// 16.16 float
    pub rate: i32,
    /// 8.8 float
    pub volume: i16,
    pub reserved_a: u16,
    pub reserved_b: [u32; 2],
    pub matrix: [i32; 9],
    pub pre_defined: [u32; 6],
    pub next_track_id: u32,
}

impl From<MpegFullBox> for MovieHeaderBox {
    fn from(value: MpegFullBox) -> Self {
        let base;
        let creation_time;
        let modification_time;
        let timescale;
        let duration;
        if value.version == 1 {
            creation_time = misc::parse_u64(&value.data[0..8]);
            modification_time = misc::parse_u64(&value.data[8..16]);
            timescale = misc::parse_u32(&value.data[16..20]);
            duration = misc::parse_u64(&value.data[20..28]);
            base = 28;
        } else {
            creation_time = misc::parse_u32(&value.data[0..4]) as u64;
            modification_time = misc::parse_u32(&value.data[4..8]) as u64;
            timescale = misc::parse_u32(&value.data[8..12]);
            duration = misc::parse_u32(&value.data[12..16]) as u64;
            base = 16;
        }
        let rate = misc::parse_i32(&value.data[base..base + 4]);
        let volume = misc::parse_i16(&value.data[base + 4..base + 6]);
        let reserved_a = misc::parse_u16(&value.data[base + 6..base + 8]);
        let reserved_b: [u32; 2] = value.data[base + 8..base + 8 + 2 * 4].chunks(4).map(misc::parse_u32).collect::<Vec<u32>>().try_into().unwrap();
        let matrix: [i32; 9] = value.data[base + 16..base + 16 + 9 * 4].chunks(4).map(misc::parse_i32).collect::<Vec<i32>>().try_into().unwrap();
        let pre_defined: [u32; 6] = value.data[base + 52..base + 52 + 6 * 4].chunks(4).map(misc::parse_u32).collect::<Vec<u32>>().try_into().unwrap();
        let next_track_id = misc::parse_u32(&value.data[76..80]);
        Self {
            flags: value.flags,
            version: value.version,
            creation_time,
            modification_time,
            timescale,
            duration,
            rate,
            volume,
            reserved_a,
            reserved_b,
            matrix,
            pre_defined,
            next_track_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrackHeaderBox {
    pub flags: [u8; 3],
    pub version: u8,
    pub creation_time: u64,
    pub modification_time: u64,
    pub track_id: u32,
    pub reserved_a: u32,
    pub duration: u64,
    pub reserved_b: [u32; 2],
    pub layer: i16,
    pub alternate_group: i16,
    /// 8.8 float
    pub volume: i16,
    pub reserved_c: u16,
    pub matrix: [i32; 9],
    // 16.16 float
    pub width: i32,
    // 16.16 float
    pub height: i32,
}

impl From<MpegFullBox> for TrackHeaderBox {
    fn from(value: MpegFullBox) -> Self {
        let creation_time;
        let modification_time;
        let track_id;
        let reserved_a;
        let duration;
        let base;
        if value.version == 1 {
            creation_time = misc::parse_u64(&value.data[0..8]);
            modification_time = misc::parse_u64(&value.data[8..16]);
            track_id = misc::parse_u32(&value.data[16..20]);
            reserved_a = misc::parse_u32(&value.data[20..24]);
            duration = misc::parse_u64(&value.data[24..32]);
            base = 32;
        } else {
            creation_time = misc::parse_u32(&value.data[0..4]) as u64;
            modification_time = misc::parse_u32(&value.data[4..8]) as u64;
            track_id = misc::parse_u32(&value.data[8..12]);
            reserved_a = misc::parse_u32(&value.data[12..16]);
            duration = misc::parse_u32(&value.data[16..20]) as u64;
            base = 20;
        }
        let reserved_b: [u32; 2] = value.data[base..base + 8].chunks(4).map(misc::parse_u32).collect::<Vec<u32>>().try_into().unwrap();
        let layer = misc::parse_i16(&value.data[base + 8..base + 10]);
        let alternate_group = misc::parse_i16(&value.data[base + 10..base + 12]);
        let volume = misc::parse_i16(&value.data[base + 12..base + 14]);
        let reserved_c = misc::parse_u16(&value.data[base + 14..base + 16]);
        let matrix: [i32; 9] = value.data[base + 16..base + 16 + 9 * 4].chunks(4).map(misc::parse_i32).collect::<Vec<i32>>().try_into().unwrap();
        let width = misc::parse_i32(&value.data[base + 52..base + 56]);
        let height = misc::parse_i32(&value.data[base + 56..base + 60]);
        Self {
            flags: value.flags,
            version: value.version,
            creation_time,
            modification_time,
            track_id,
            reserved_a,
            duration,
            reserved_b,
            layer,
            alternate_group,
            volume,
            reserved_c,
            matrix,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EditListEntry {
    pub segment_duration: u64,
    pub media_time: i64,
    pub media_rate_integer: i16,
    pub media_rate_fraction: i16,
}

#[derive(Debug, Clone)]
pub struct EditListBox {
    pub version: u8,
    pub entry_count: u32,
    pub entries: Vec<EditListEntry>,
}

impl From<MpegFullBox> for EditListBox {
    fn from(value: MpegFullBox) -> Self {
        let entry_count = misc::parse_u32(&value.data[0..4]);
        let mut entries = vec![];
        let mut base = 4;
        for _ in 0..entry_count {
            let segment_duration;
            let media_time;
            if value.version == 1 {
                segment_duration = misc::parse_u64(&value.data[base..base + 8]);
                media_time = misc::parse_i64(&value.data[base + 8..base + 16]);
                base += 16;
            } else {
                segment_duration = misc::parse_u32(&value.data[base..base + 4]) as u64;
                media_time = misc::parse_i32(&value.data[base + 4..base + 8]) as i64;
                base += 8;
            }
            let media_rate_integer = misc::parse_i16(&value.data[base..base + 2]);
            let media_rate_fraction = misc::parse_i16(&value.data[base + 2..base + 4]);
            entries.push(EditListEntry {
                media_time,
                segment_duration,
                media_rate_integer,
                media_rate_fraction,
            })
        }
        Self {
            version: value.version,
            entry_count,
            entries,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EditBox {
    pub edit_list: Option<EditListBox>,
}

#[derive(Debug, Clone)]
pub struct TrackBox {
    pub track_header: TrackHeaderBox,
    pub edit_lists: Option<EditBox>,
}

// TODO: parse TrackBox
