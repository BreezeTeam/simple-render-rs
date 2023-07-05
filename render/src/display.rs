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
    let mut window = Window::new(
        "Image Viewer",
        width,
        height,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
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
                let [r, g, b, _a] = pixel.0;
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
#[macro_export]
macro_rules! display_images {
    ( $duration:expr, $($image:expr),*  ) => {
        let exit_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        $(
        let image = std::sync::Arc::new(std::sync::Mutex::new(image::DynamicImage::from($image.clone())));
        let width = $image.width() as usize;
        let height = $image.height() as usize;
        std::thread::spawn({
            let exit_flag = std::sync::Arc::clone(&exit_flag);
            let image = std::sync::Arc::clone(&image);
            move || {
                $crate::display::display_image(
                        image,
                        width,
                        height,
                        exit_flag
                )
            }
        });
    )*

    std::thread::sleep(std::time::Duration::from_secs($duration));
        exit_flag.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::thread;
    use std::time::Duration;

    /// 测试宏
    #[test]
    fn test_display_many_image_use_macro() {
        let image1 = DynamicImage::new_rgba8(100, 100);
        let image2 = DynamicImage::new_rgba8(200, 200);
        let image3 = DynamicImage::new_rgba8(300, 300);
        display_images!(2, image1, image2, image3);
    }

    /// 显示多个图像
    #[test]
    fn test_display_many_image() {
        let image1 = DynamicImage::new_rgba8(100, 100);
        let image2 = DynamicImage::new_rgba8(200, 200);
        let image3 = DynamicImage::new_rgba8(300, 300);

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
    }

    /// 测试宏
    #[test]
    fn test_display_one_image_use_macro() {
        let image = DynamicImage::new_rgba8(100, 100);
        display_images!(2, image, image, image);
    }
}
