use image::{DynamicImage, Rgba, RgbaImage};

/// 水平翻转
fn flip_horizontally(image: &mut RgbaImage) {
    let width = image.width();
    let height = image.height();

    for y in 0..height {
        for x in 0..width / 2 {
            let x2 = width - 1 - x;
            let temp = image.get_pixel(x, y).clone();
            image.put_pixel(x, y, *image.get_pixel(x2, y));
            image.put_pixel(x2, y, temp);
        }
    }
}

/// 垂直翻转
fn flip_vertically(image: &mut RgbaImage) {
    let width = image.width();
    let height = image.height();

    for y in 0..height / 2 {
        for x in 0..width {
            let y2 = height - 1 - y;
            let temp = image.get_pixel(x, y).clone();
            image.put_pixel(x, y, *image.get_pixel(x, y2));
            image.put_pixel(x, y2, temp);
        }
    }
}

/// 清空图像
fn clear(image: &mut RgbaImage) {
    for pixel in image.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]);
    }
}

/// 缩放图像
fn scale(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
}
/// 对图像进行transform
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // 读取cargo.toml 文件所在地址
    let resource_path = format!("{}/../resource", env!("CARGO_MANIFEST_DIR"));
    log::info!("resource_path :{}", &resource_path);

    // 读取原始的 TGA 文件
    let original_image = image::open(&format!("{}/study/img2.tga", resource_path))
        .expect("Failed to read TGA file")
        .to_rgba8();

    // 水平翻转
    let mut flipped_horizontally = original_image.clone();
    flip_horizontally(&mut flipped_horizontally);
    flipped_horizontally
        .save(&format!("{}/study/img2-horizontally.tga", resource_path))
        .expect("Failed to write horizontally TGA file");

    // 垂直翻转
    let mut flipped_vertically = original_image.clone();
    flip_vertically(&mut flipped_vertically);
    flipped_vertically
        .save(&format!("{}/study/img2-vertically.tga", resource_path))
        .expect("Failed to write vertically TGA file");

    // scale
    let encoded_image = image::DynamicImage::ImageRgba8(original_image.clone());
    scale(&encoded_image, 50, 50)
        .to_rgba8()
        .save(&format!("{}/study/img2-scale.tga", resource_path))
        .expect("Failed to write scale TGA file");

    // clear
    let mut clear_image = original_image.clone();
    clear(&mut clear_image);
    clear_image
        .save(&format!("{}/study/img2-clear.tga", resource_path))
        .expect("Failed to write clear TGA file");
}
