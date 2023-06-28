use std::{fs::File, io};

use image::{ImageBuffer, ImageFormat, Rgba};

/// 将 ImageBuffer 写入到 tga文件中
fn write_tga_file(image: &ImageBuffer<Rgba<u8>, Vec<u8>>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let encoded_image = image::DynamicImage::ImageRgba8(image.clone());
    encoded_image
        .write_to(&mut file, ImageFormat::Tga)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}
/// 读取tga文件，并且加载为ImageBuffer
fn read_tga_file(file_path: &str) -> io::Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let image = image::open(file_path).unwrap().to_rgba8();
    Ok(image)
}
///使用image存储tga格式的数据
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // 读取cargo.toml 文件所在地址
    let resource_path = format!("{}/../resource", env!("CARGO_MANIFEST_DIR"));
    log::info!("resource_path :{}", &resource_path);

    // 读取原始的 TGA 文件
    let original_image = read_tga_file(&format!("{}/study/img2.tga", resource_path))
        .expect("Failed to read TGA file");

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

    // Save the image
    write_tga_file(
        &modified_image,
        &format!("{}/study/img2-circle.tga", resource_path),
    )
    .expect("Failed to write TGA file");
}
