use std::fs::File;

fn main() {
    let mut file = File::open("ref/test.mp4").unwrap();
    while let Ok(bx) = mp4::read_box(&mut file) {
        eprintln!("Read box: size = {}, name = {}", bx.size, bx.name);
        match bx.name.as_str() {
            "ftyp" => {
                let ftyp = mp4::FileTypeBox::from(bx);
                eprint!("  Major brand = {}\n  Minor version = {}\n  Compatible brands = {:?}\n",
                    ftyp.major_brand,
                    ftyp.minor_version,
                    ftyp.compatible_brands
                );
            },
            "moov" => {
                let mut reader = std::io::Cursor::new(bx.data);
                while let Ok(sbx) = mp4::read_box(&mut reader) {
                    eprintln!("  Read nested box: size = {}, name = {}", sbx.size, sbx.name);
                    if sbx.name == "mvhd" {
                        let mvhd = mp4::MovieHeaderBox::from(mp4::MpegFullBox::from(sbx));
                        eprint!("    Creation time = {}\n    Modification time = {}\n    Timescale = {}\n    Duration = {:?}\n    Rate = {}\n    Volume = {}\n",
                            mvhd.creation_time,
                            mvhd.modification_time,
                            mvhd.timescale,
                            mp4::misc::parse_duration(mvhd.duration, mvhd.timescale),
                            mvhd.rate as f32 / 65536.0,
                            mvhd.volume as f32 / 256.0,
                        );
                    }
                }
            },
            _ => (),
        }
    }
}
