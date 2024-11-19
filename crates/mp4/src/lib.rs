#[derive(Debug)]
pub struct MpegBox {
    pub size: u64,
    pub name: String,
    pub usertype: Option<[u8; 16]>,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct MpegFullBox {
    pub size: u64,
    pub name: String,
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
    let mut buf = [0u8; 4];
    input.read_exact(&mut buf)?;
    let mut size = u32::from_be_bytes(buf) as u64;  // big endian
    input.read_exact(&mut buf)?;
    let name = String::from_utf8(buf.into())?;
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

#[derive(Debug)]
pub struct FileTypeBox {
    pub major_brand: String,
    pub minor_version: u32,
    pub compatible_brands: Vec<String>,
}

impl From<MpegBox> for FileTypeBox {
    fn from(value: MpegBox) -> Self {
        Self {
            major_brand: String::from_utf8(value.data[0..4].into()).unwrap(),
            minor_version: u32::from_be_bytes(value.data[4..8].try_into().unwrap()),
            compatible_brands: value.data[8..].chunks(4).map(|x| String::from_utf8(x.into()).unwrap()).collect(),
        }
    }
}
