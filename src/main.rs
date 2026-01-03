use crate::{
    filters::{box_blur::BoxBlur, flip_h::FlipH, flip_v::FlipV, negative::Negative},
    images::{Image, bitmap::Bitmap},
};

mod filters;
mod images;

fn main() {
    let mut bitmap = Bitmap::open("./images/tower.bmp").unwrap();

    /*
    let _ = bitmap.filter(FlipH::full());
    let _ = bitmap.save("./images/clone1.bmp").unwrap();

    let mut bitmap = Bitmap::open("./images/tower.bmp").unwrap();
    let _ = bitmap.filter(FlipH::rect((0, 0), (600, 200)));
    let _ = bitmap.save("./images/clone2.bmp").unwrap();

    let mut bitmap = Bitmap::open("./images/negative.bmp").unwrap();
    let _ = bitmap.filter(Negative);
    let _ = bitmap.save("./images/negative2.bmp").unwrap();
    */

    let _ = bitmap.filter(FlipV::full());
    let _ = bitmap.save("./images/clone1.bmp").unwrap();

    let mut bitmap = Bitmap::open("./images/tower.bmp").unwrap();
    let _ = bitmap.filter(FlipV::rect((0, 0), (600, 200)));
    let _ = bitmap.filter(Negative);
    let _ = bitmap.save("./images/clone2.bmp").unwrap();

    let mut bitmap = Bitmap::open("./images/tower.bmp").unwrap();
    let _ = bitmap.filter(BoxBlur(1));
    let _ = bitmap.save("./images/blur1.bmp").unwrap();

    let mut bitmap = Bitmap::open("./images/tower.bmp").unwrap();
    let _ = bitmap.filter(BoxBlur(2));
    let _ = bitmap.save("./images/blur2.bmp").unwrap();
}
