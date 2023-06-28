use std::{fs::File, io};

use image::{ImageBuffer, ImageFormat, Rgba};

///使用image存储tga格式的数据
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // 读取cargo.toml 文件所在地址
    let resource_path = format!("{}/../resource", env!("CARGO_MANIFEST_DIR"));
    log::info!("resource_path :{}", &resource_path);

    // Create a new ImageBuffer with width: imgx and height: imgy
    let mut imgbuf = ImageBuffer::new(800, 800);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = Rgba([r, 0, b, 255]);
    }
    imgbuf.save(&format!("{}/study/img2.tga", resource_path)).expect("Failed to write TGA file");
}
