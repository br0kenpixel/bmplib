use bmplib::BitmapImage;
use std::path::Path;

fn main() {
    let image_path = Path::new("examples/image.bmp");
    let image = BitmapImage::from_file(image_path).unwrap();

    println!("{image:#?}");
}
