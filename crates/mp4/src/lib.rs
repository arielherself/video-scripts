#[derive(Debug)]
pub struct MpegBox {
    pub size: u64,
    pub name: String,
    pub usertype: Option<[u8; 16]>,
    pub data: Vec<u8>,
}


pub fn read_box(input: &mut dyn std::io::Read) -> Result<MpegBox, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 4];
    input.read_exact(&mut buf)?;
    let mut size = u32::from_be_bytes(buf) as u64;  // big endian
    input.read_exact(&mut buf)?;
    let name = String::from_utf8(buf.into())?;
    let mut data = vec!();
    if size == 1 {
        // largesize
        let mut largebuf = [0u8; 8];
        input.read_exact(&mut largebuf)?;
        size = u64::from_be_bytes(largebuf);
    }
    let mut usertype = None;
    if name == "uuid" {
        let mut buf = [0u8; 16];
        input.read_exact(&mut buf)?;
        usertype = Some(buf);
    }
    if size == 0 {
        // read till EOF
        input.read_to_end(&mut data)?;
        size = data.len() as u64;
    } else {
        data.resize(size as usize, 0u8);
        input.read_exact(&mut data)?;
    }

    Ok(MpegBox { size, name, usertype, data })
}

