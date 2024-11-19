#[derive(Debug)]
pub struct MpegBox {
    pub size: u32,
    pub name: String,
    pub data: Vec<u8>,
}


pub fn read_box(input: &mut dyn std::io::Read) -> Result<MpegBox, Box<dyn std::error::Error>> {
    let mut buf = [0u8; 4];
    input.read_exact(&mut buf)?;
    let mut size = u32::from_be_bytes(buf);  // big endian
    input.read_exact(&mut buf)?;
    let name = String::from_utf8(buf.into())?;
    let mut data = vec!();
    if size == 0 {
        // read till EOF
        input.read_to_end(&mut data)?;
        size = data.len() as u32;
    } else {
        data.resize(size as usize, 0u8);
        input.read_exact(&mut data)?;
    }

    Ok(MpegBox { size, name, data })
}

