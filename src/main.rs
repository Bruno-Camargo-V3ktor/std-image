use crate::{
    filters::*,
    images::{Image, bitmap::Bitmap},
};

mod filters;
mod images;

fn main() {
    let mut bitmap = Bitmap::open("./images/stadium.bmp").unwrap();

    let _ = bitmap.filter(edge_detection::EdgeDetection::grayscale_with_limit(20));
    let _ = bitmap.save("./images/edges.bmp").unwrap();
}
