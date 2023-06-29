use image::{
    DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageFormat, Rgba, RgbaImage,
};
use render::helper::*;
use std::{
    fs::File,
    io,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

/// line绘制算法
/// https://zh.wikipedia.org/zh-hans/%E5%B8%83%E9%9B%B7%E6%A3%AE%E6%BC%A2%E5%A7%86%E7%9B%B4%E7%B7%9A%E6%BC%94%E7%AE%97%E6%B3%95
fn line(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    color: Rgba<u8>,
) {
    let mut steep = false;
    let mut x0 = x0;
    let mut y0 = y0;
    let mut x1 = x1;
    let mut y1 = y1;

    let width = image.width() as i32;
    let height = image.height() as i32;

    if (x0 - x1).abs() < (y0 - y1).abs() {
        steep = true;
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;

    for x in x0..=x1 {
        if steep {
            if y >= 0 && y < height && x >= 0 && x < width {
                image.put_pixel(y as u32, x as u32, color);
            }
        } else {
            if x >= 0 && x < width && y >= 0 && y < height {
                image.put_pixel(x as u32, y as u32, color);
            }
        }

        error2 += derror2;

        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}

/// 绘制line
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // 读取cargo.toml 文件所在地址
    let resource_path = format!("{}/../resource", env!("CARGO_MANIFEST_DIR"));
    log::info!("resource_path :{}", &resource_path);

    // 读取原始的 TGA 文件
    let mut original_image = image::open(&format!("{}/study/img2.tga", resource_path))
        .expect("Failed to read TGA file")
        .to_rgba8();

    let red = Rgba([255, 0, 0, 255]); // 红色
    let white = Rgba([255, 255, 255, 255]); // 白色
    line(100, 0, 300, 10000, &mut original_image, red);
    line(13, 20, 80, 40, &mut original_image, white);
    line(20, 13, 40, 80, &mut original_image, red);
    // line2(80, 40, 13, 20, &mut original_image, red);
    original_image
        .save(&format!("{}/study/img-display.png", resource_path))
        .expect("Failed to write clear TGA file");
    let arc_image = Arc::new(Mutex::new(image::DynamicImage::ImageRgba8(
        original_image.clone(),
    )));

    display_image(arc_image, 800, 800, Arc::new(AtomicBool::new(false)));
}
