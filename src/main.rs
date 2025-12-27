use crate::images::bitmap::Bitmap;
mod images;

fn main() {
    let _bitmap = Bitmap::open("./images/tower.bmp").unwrap();
}
