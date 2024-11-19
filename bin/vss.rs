use std::fs::File;

fn main() {
    let mut file = File::open("ref/test.mp4").unwrap();
    while let Ok(bx) = mp4::read_box(&mut file) {
        eprintln!("Read box: size = {}, name = {}", bx.size, bx.name);
    }
}
