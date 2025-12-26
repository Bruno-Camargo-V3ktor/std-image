use crate::images::bitmap::Bitmap;
mod images;

fn main() {
    let bitmap = Bitmap::open("./images/tower.bmp").unwrap();

    //println!("{:#?}", bitmap.width);
    //println!("{:#?}", bitmap.height);
}
