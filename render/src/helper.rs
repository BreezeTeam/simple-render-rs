use image::DynamicImage;
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
        if exit_flag.load(Ordering::Relaxed) {
            break;
        }
        // 获取图像数据的读锁
        let image_data = image.lock().unwrap();

        // 转换图像数据为 RGBA 格式
        let rgba_image = image_data.to_rgba8();

        // 将图像像素值转换为u32格式
        let u32_image: Vec<u32> = rgba_image
            .pixels()
            .map(|pixel| {
                let [r, g, b, a] = pixel.0;
                ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
            })
            .collect();

        // 更新窗口显示
        window
            .update_with_buffer(&u32_image, width, height)
            .expect("Failed to update window");
    }
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
        thread::sleep(Duration::from_secs(2)); // 2秒的等待时间示例
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
}
