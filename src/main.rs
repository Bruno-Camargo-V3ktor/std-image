use crate::{
    filters::flip_h::FlipH,
    images::{Image, bitmap::Bitmap},
};

mod filters;
mod images;

fn main() {
    let mut bitmap = Bitmap::open("./images/tower.bmp").unwrap();

    let _ = bitmap.filter(FlipH::full());
    let _ = bitmap.save("./images/clone1.bmp").unwrap();

    let mut bitmap = Bitmap::open("./images/tower.bmp").unwrap();
    let _ = bitmap.filter(FlipH::rect((0, 0), (600, 200)));
    let _ = bitmap.save("./images/clone2.bmp").unwrap();
}
