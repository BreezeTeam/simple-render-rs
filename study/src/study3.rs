use std::{fs::File, io};

use image::{ImageBuffer, ImageFormat, Rgba};

/// 读取原始的tga图像
/// 添加一个红色的圆
/// 然后在存储为tga格式
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // 读取cargo.toml 文件所在地址
    let resource_path = format!("{}/../resource", env!("CARGO_MANIFEST_DIR"));
    log::info!("resource_path :{}", &resource_path);

    // 读取原始的 TGA 文件
    let original_image = image::open(&format!("{}/study/img2.tga", resource_path)).expect("Failed to read TGA file").to_rgba8();
    // 在图像上添加一个圆形
    let mut modified_image = original_image.clone();
    let (width, height) = modified_image.dimensions();
    let center_x = width / 2;
    let center_y = height / 2;
    let radius = (width.min(height) / 4) as f32;
    for (x, y, pixel) in modified_image.enumerate_pixels_mut() {
        let distance =
            ((x as f32 - center_x as f32).powi(2) + (y as f32 - center_y as f32).powi(2)).sqrt();
        if distance <= radius {
            *pixel = Rgba([255, 0, 0, 255]); // 设置为红色
        }
    }

    modified_image.save(&format!("{}/study/img2-circle.tga", resource_path)).expect("Failed to write TGA file");
}
