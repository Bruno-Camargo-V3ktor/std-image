use crate::images::{Image, bitmap::Bitmap};
mod images;

fn main() {
    let mut bitmap = Bitmap::open("./images/tower.bmp").unwrap();
    bitmap.flip_h();
    let _ = bitmap.save("./images/clone.bmp").unwrap();
}
