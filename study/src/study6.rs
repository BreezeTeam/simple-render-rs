use image::{
    DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageFormat, Rgba, RgbaImage,
};
use render::helper::*;
use std::{
    fs::File,
    io,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

/// 简陋的line绘制算法
fn line(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    color: Rgba<u8>,
) {
    for t in (0..=100).map(|t| t as f32 / 100.0) {
        let x = (x0 as f32 + (x1 - x0) as f32 * t) as i32;
        let y = (y0 as f32 + (y1 - y0) as f32 * t) as i32;
        image.put_pixel(x as u32, y as u32, color);
    }
}

fn line2(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, color: Rgba<u8>) {
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

    let color = Rgba([255, 0, 0, 255]); // 红色，RGBA格式
                                        // line(100, 100, 700, 500, &mut original_image, color);
    line2(100, 0, 300, 10000, &mut original_image, color);

    original_image
        .save(&format!("{}/study/img-display.png", resource_path))
        .expect("Failed to write clear TGA file");
    let arc_image = Arc::new(Mutex::new(image::DynamicImage::ImageRgba8(
        original_image.clone(),
    )));

    display_image(arc_image, 800, 800, Arc::new(AtomicBool::new(false)));
}
