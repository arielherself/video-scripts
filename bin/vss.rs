use std::fs::File;

struct MpegFileWalker {
    timescale: u32,
}

impl MpegFileWalker {
    fn box_info_handler(&mut self, bx: mp4::MpegBox, tabs: usize) -> bool {
        let padding = vec!["  "; tabs].concat();
        match bx.name.as_str() {
            "ftyp" => {
                let ftyp = mp4::FileTypeBox::from(bx);
                eprint!("{padding}Major brand = {}\n{padding}Minor version = {}\n{padding}Compatible brands = {:?}\n",
                    ftyp.major_brand,
                    ftyp.minor_version,
                    ftyp.compatible_brands,
                );
                false
            },
            "moov" => true,
            "mvhd" => {
                let mvhd = mp4::MovieHeaderBox::from(mp4::MpegFullBox::from(bx));
                self.timescale = mvhd.timescale;
                eprint!("{padding}Creation time = {}\n{padding}Modification time = {}\n{padding}Timescale = {}\n{padding}Duration = {:?}\n{padding}Rate = {}\n{padding}Volume = {}\n",
                    mvhd.creation_time,
                    mvhd.modification_time,
                    mvhd.timescale,
                    mp4::misc::parse_duration(mvhd.duration, mvhd.timescale),
                    mvhd.rate as f32 / 65536.0,
                    mvhd.volume as f32 / 256.0,
                );
                false
            },
            "trak" => true,
            "tkhd" => {
                let tkhd = mp4::TrackHeaderBox::from(mp4::MpegFullBox::from(bx));
                eprint!("{padding}Creation time = {}\n{padding}Modification time = {}\n{padding}Track ID = {}\n{padding}Duration = {:?}\n{padding}Volume = {}\n{padding}Width = {}\n{padding}Height = {}\n",
                    tkhd.creation_time,
                    tkhd.modification_time,
                    tkhd.track_id,
                    mp4::misc::parse_duration(tkhd.duration, self.timescale),
                    tkhd.volume as f32 / 256.0,
                    tkhd.width as f32 / 65536.0,
                    tkhd.height as f32 / 65536.0,
                );
                false
            }
            _ => false,
        }
    }

    fn print_box_info(&mut self, bx: mp4::MpegBox, tabs: usize) {
        let padding = vec!["  "; tabs].concat();
        eprintln!("{padding}Read box: size = {}, name = {}", bx.size, bx.name);
        if self.box_info_handler(bx.to_owned(), tabs + 1) {
            let mut reader = std::io::Cursor::new(bx.data);
            while let Ok(sbx) = mp4::read_box(&mut reader) {
                self.print_box_info(sbx, tabs + 1);
            }
        }
    }

    fn print_file_info(&mut self, reader: &mut dyn std::io::Read) {
        while let Ok(bx) = mp4::read_box(reader) {
            self.print_box_info(bx, 0);
        }
    }
}


fn main() {
    let mut file = File::open("ref/test.mp4").unwrap();
    let mut walker = MpegFileWalker { timescale: 1 };
    walker.print_file_info(&mut file);
}
