use std::fs::File;

fn main() {
    let mut file = File::open("ref/test.mp4").unwrap();
    while let Ok(bx) = mp4::read_box(&mut file) {
        eprintln!("Read box: size = {}, name = {}", bx.size, bx.name);
        match bx.name.as_str() {
            "ftyp" => {
                let ftyp = mp4::FileTypeBox::from(bx);
                eprint!("  Major brand = {}\n  Minor version = {}\n  Compatible brands = {:?}\n", ftyp.major_brand, ftyp.minor_version, ftyp.compatible_brands);
            },
            "moov" => {
                let mut reader = std::io::Cursor::new(bx.data);
                while let Ok(sbx) = mp4::read_box(&mut reader) {
                    eprintln!("  Read nested box: size = {}, name = {}", sbx.size, sbx.name);
                }
            },
            _ => (),
        }
    }
}
