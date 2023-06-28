use std::{fs::File, io};

use image::{DynamicImage, GenericImage, ImageBuffer, ImageFormat, Rgba};

/// 创建一个空白图片
/// 修改一个像素点
/// 使用image存储png格式的数据
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // 读取cargo.toml 文件所在地址
    let resource_path = format!("{}/../resource", env!("CARGO_MANIFEST_DIR"));
    log::info!("resource_path :{}", &resource_path);

    // Create a new ImgBuf with width: imgx and height: imgy
    let original_image = image::ImageBuffer::new(800, 800);

    let mut modified_image: ImageBuffer<Rgba<u8>, Vec<u8>> = original_image.clone();
    let (width, height) = modified_image.dimensions();
    let center_x = width / 2;
    let center_y = height / 2;
    // 在图像上添加一个点
    modified_image.put_pixel(center_x, center_y, Rgba([255, 0, 0, 255]));
    //如果这里不用rgb8，那么会丢失通道信息，导致所有像素的alpha都是255
    modified_image.save(&format!("{}/study/img2-point.png", resource_path)).unwrap();
}
