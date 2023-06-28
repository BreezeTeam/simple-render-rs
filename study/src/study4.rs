use std::{fs::File, io};

use image::{DynamicImage, GenericImage, ImageBuffer, ImageFormat, Rgba};

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

fn set_pixel(image: &mut DynamicImage, x: u32, y: u32, pixel: Rgba<u8>) {
    if x < image.width() && y < image.height() {
        image.put_pixel(x, y, pixel);
    }
}

///使用image存储tga格式的数据
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // 读取cargo.toml 文件所在地址
    let resource_path = format!("{}/../resource", env!("CARGO_MANIFEST_DIR"));
    log::info!("resource_path :{}", &resource_path);

    // // 读取原始的 TGA 文件
    // let original_image = read_tga_file(&format!("{}/study/img2.tga", resource_path))
    //     .expect("Failed to read TGA file");

    // Create a new ImgBuf with width: imgx and height: imgy
    let original_image = image::ImageBuffer::new(800, 800);

    let modified_image: ImageBuffer<Rgba<u8>, Vec<u8>> = original_image.clone();
    let (width, height) = modified_image.dimensions();
    let center_x = width / 2;
    let center_y = height / 2;

    log::info!("center_x:{}，center_y:{}", center_x, center_y);

    let mut dyimg = image::DynamicImage::ImageRgba8(modified_image);
    // // 在图像上添加一个点
    // set_pixel(
    //     &mut dyimg,
    //     center_x,
    //     center_y,
    //     Rgba([255, 0, 0, 255]),
    // );

    // Save the image as “fractal.png”, the format is deduced from the path
    dyimg.to_rgb8()
        .save(&format!("{}/study/img2-point.png", resource_path))
        .unwrap();
    // Save the image
    // write_tga_file(
    //     &modified_image,
    //     &format!("{}/study/img2-point.tga", resource_path),
    // )
    // .expect("Failed to write TGA file");
}
