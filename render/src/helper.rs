use image::{DynamicImage, Rgba, RgbaImage};
use minifb::{Key, Window, WindowOptions};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// 创建窗口并且显示图片
pub fn display_image(
    image: Arc<Mutex<DynamicImage>>,
    width: usize,
    height: usize,
    exit_flag: Arc<AtomicBool>,
) {
    // 创建窗口
    let mut window = Window::new("Image Viewer", width, height, WindowOptions::default())
        .expect("Failed to create window");

    // 将图像数据传递给窗口进行显示
    while window.is_open() && !window.is_key_down(Key::Escape) && !exit_flag.load(Ordering::Relaxed)
    {
        // 获取图像数据的读锁
        let image_data = image.lock().unwrap();

        // 转换图像数据为 RGBA 格式
        let rgba_image = image_data.to_rgba8();

        // 将图像像素值转换为u32格式
        let u32_image: Vec<u32> = rgba_image
            .pixels()
            .map(|pixel| {
                let [r, g, b, a] = pixel.0;
                // 通道顺序为RGBA，即先红色（R），然后绿色（G），然后蓝色（B），最后透明度（A）。
                ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
            })
            .collect();

        // 更新窗口显示
        if let Err(_) = window.update_with_buffer(&u32_image, width, height) {
            eprintln!("Failed to update window");
            break;
        }
    }
}

/// 水平翻转
pub fn flip_horizontally(image: &mut RgbaImage) {
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
pub fn flip_vertically(image: &mut RgbaImage) {
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
pub fn clear(image: &mut RgbaImage) {
    for pixel in image.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]);
    }
}

/// 缩放图像
pub fn scale(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::DerefMut;
    use std::thread;

    use image::{ImageBuffer, ImageFormat, Rgb, Rgba};
    use std::time::Duration;

    /// 显示多个图像
    #[test]
    fn test_display_many_image() {
        let image1 = DynamicImage::new_rgba8(100, 100);
        let image2 = DynamicImage::new_rgba8(200, 200);
        let image3 = DynamicImage::new_rgba8(300, 300);

        // let mut rgba_image = image2.to_rgba8();
        // // 修改图像数据
        // let (width, height) = rgba_image.dimensions();
        // let center_x = width / 2;
        // let center_y = height / 2;
        // let radius = (width.min(height) / 4) as f32;
        // for (x, y, pixel) in rgba_image.enumerate_pixels_mut() {
        //     let distance = ((x as f32 - center_x as f32).powi(2) + (y as f32 - center_y as f32).powi(2)).sqrt();
        //     if distance <= radius {
        //         *pixel = Rgba([255, 0, 0, 255]); // 设置圆形为红色
        //     }
        // }

        //创建一个共享变量
        let exit_flag = Arc::new(AtomicBool::new(false));

        let width1 = image1.width() as usize;
        let height1 = image1.height() as usize;
        let thread1 = thread::spawn({
            let exit_flag = Arc::clone(&exit_flag);
            move || display_image(Arc::new(Mutex::new(image1)), width1, height1, exit_flag)
        });

        let width2 = image2.width() as usize;
        let height2 = image2.height() as usize;
        let thread2 = thread::spawn({
            let exit_flag = Arc::clone(&exit_flag);
            move || display_image(Arc::new(Mutex::new(image2)), width2, height2, exit_flag)
        });

        let width3 = image3.width() as usize;
        let height3 = image3.height() as usize;
        let thread3 = thread::spawn({
            let exit_flag = Arc::clone(&exit_flag);
            move || display_image(Arc::new(Mutex::new(image3)), width3, height3, exit_flag)
        });

        // 在断言后的代码块中添加适当的等待时间
        // 2秒的等待时间示例
        thread::sleep(Duration::from_secs(2));

        // 然后将其设置为可以关闭
        exit_flag.store(true, Ordering::Relaxed);

        //断言
        assert!(thread1.join().is_ok());
        assert!(thread2.join().is_ok());
        assert!(thread3.join().is_ok());
    }

    // 显示同一个图像
    #[test]
    fn test_display_one_image() {
        // 创建一个示例图像
        let image = Arc::new(Mutex::new(DynamicImage::new_rgba8(100, 100)));

        // // 修改图像数据
        // {
        //
        //     let mut image_data = image.lock().unwrap();
        //     let mut rgba_image = image_data.to_rgba8();
        //     // 修改图像数据
        //     let (width, height) = rgba_image.dimensions();
        //     let center_x = width / 2;
        //     let center_y = height / 2;
        //     let radius = (width.min(height) / 4) as f32;
        //     for (x, y, pixel) in rgba_image.enumerate_pixels_mut() {
        //         let distance = ((x as f32 - center_x as f32).powi(2) + (y as f32 - center_y as f32).powi(2)).sqrt();
        //         if distance <= radius {
        //             *pixel = Rgba([255, 0, 0, 255]); // 设置圆形为红色
        //         }
        //     }
        // }
        //创建一个共享变量
        let exit_flag = Arc::new(AtomicBool::new(false));

        // 创建线程来显示图像
        let thread1 = thread::spawn({
            let exit_flag = Arc::clone(&exit_flag);
            let image = Arc::clone(&image);
            move || display_image(image, 100, 100, exit_flag)
        });

        let thread2 = thread::spawn({
            let exit_flag = Arc::clone(&exit_flag);
            let image = Arc::clone(&image);
            move || display_image(image, 100, 100, exit_flag)
        });

        let thread3 = thread::spawn({
            let exit_flag = Arc::clone(&exit_flag);
            let image = Arc::clone(&image);
            move || display_image(image, 100, 100, exit_flag)
        });

        // 在断言后的代码块中添加适当的等待时间
        thread::sleep(Duration::from_secs(2)); // 2秒的等待时间示例
                                               // 然后将其设置为可以关闭
        exit_flag.store(true, Ordering::Relaxed);

        // 断言
        assert!(thread1.join().is_ok());
        assert!(thread2.join().is_ok());
        assert!(thread3.join().is_ok());

        // // 创建多个线程来同时修改图像
        // let threads: Vec<_> = (0..4)
        //     .map(|i| {
        //         let image = Arc::clone(&image);
        //         thread::spawn(move || {
        //             // 获取图像数据的锁
        //             let mut image_data = image.lock().unwrap();
        //             let mut rgba_image = image_data.to_rgba8();
        //             // 修改图像数据
        //             let (width, height) = rgba_image.dimensions();
        //             let x = i * 10;
        //             let y = i * 10;
        //             for dy in 0..10 {
        //                 for dx in 0..10 {
        //                     let pixel_x = x + dx;
        //                     let pixel_y = y + dy;
        //                     if pixel_x < width && pixel_y < height {
        //                         rgba_image.put_pixel(pixel_x, pixel_y, Rgba([255, 0, 0, 255]));
        //                     }
        //                 }
        //             }
        //         })
        //     })
        //     .collect();
        //
        // // 等待所有线程完成
        // for thread in threads {
        //     thread.join().unwrap();
        // }
    }

    #[test]
    fn test_flip_horizontally() {
        let mut image = RgbaImage::from_fn(2, 2, |x, y| Rgba([x as u8, y as u8, 0, 255]));

        flip_horizontally(&mut image);

        assert_eq!(image.get_pixel(0, 0), &Rgba([1, 0, 0, 255]));
        assert_eq!(image.get_pixel(1, 0), &Rgba([0, 0, 0, 255]));
        assert_eq!(image.get_pixel(0, 1), &Rgba([1, 1, 0, 255]));
        assert_eq!(image.get_pixel(1, 1), &Rgba([0, 1, 0, 255]));
    }
    #[test]
    fn test_flip_vertically() {
        let mut image = RgbaImage::from_fn(2, 2, |x, y| Rgba([x as u8, y as u8, 0, 255]));

        flip_vertically(&mut image);

        assert_eq!(image.get_pixel(0, 0), &Rgba([0, 1, 0, 255]));
        assert_eq!(image.get_pixel(1, 0), &Rgba([1, 1, 0, 255]));
        assert_eq!(image.get_pixel(0, 1), &Rgba([0, 0, 0, 255]));
        assert_eq!(image.get_pixel(1, 1), &Rgba([1, 0, 0, 255]));
    }
    #[test]
    fn test_clear() {
        let mut image = RgbaImage::from_pixel(2, 2, Rgba([1, 2, 3, 4]));
        clear(&mut image);
        assert_eq!(image.get_pixel(0, 0), &Rgba([0, 0, 0, 0]));
        assert_eq!(image.get_pixel(1, 0), &Rgba([0, 0, 0, 0]));
        assert_eq!(image.get_pixel(0, 1), &Rgba([0, 0, 0, 0]));
        assert_eq!(image.get_pixel(1, 1), &Rgba([0, 0, 0, 0]));
    }
    #[test]
    fn test_scale() {
        let image = RgbaImage::from_pixel(2, 2, Rgba([1, 2, 3, 4]));
        let scaled_image = scale(&image::DynamicImage::ImageRgba8(image.clone()), 4, 4);
        assert_eq!(scaled_image.width(), 4);
        assert_eq!(scaled_image.height(), 4);
    }
}
