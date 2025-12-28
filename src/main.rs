use crate::images::bitmap::Bitmap;
mod images;

fn main() {
    let mut bitmap = Bitmap::open("./images/tower.bmp").unwrap();
    let _ = bitmap.save("./images/clone.bmp").unwrap();
}
