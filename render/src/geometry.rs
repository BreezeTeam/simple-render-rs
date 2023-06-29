use image::{ImageBuffer, Rgba};

/// line绘制算法
/// https://zh.wikipedia.org/zh-hans/%E5%B8%83%E9%9B%B7%E6%A3%AE%E6%BC%A2%E5%A7%86%E7%9B%B4%E7%B7%9A%E6%BC%94%E7%AE%97%E6%B3%95
pub fn line(
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

#[cfg(test)]
mod tests {
    use super::super::display_images;
    use super::*;
    use image::ImageBuffer;

    #[test]
    fn test_normal_line() {
        let mut image = ImageBuffer::new(100, 100);
        // 正常绘制
        line(10, 10, 50, 50, &mut image, Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(10, 10), &Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(30, 30), &Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(50, 50), &Rgba([255, 0, 0, 255]));
        display_images!(2, image);
    }
    #[test]
    fn test_abnormal_input() {
        let mut image = ImageBuffer::new(100, 100);
        // 起点和终点坐标相同
        line(50, 50, 50, 50, &mut image, Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(50, 50), &Rgba([255, 0, 0, 255]));

        // 起点和终点坐标超出图像范围
        line(1, 1, 500, 500, &mut image, Rgba([0, 255, 0, 255]));
        assert_eq!(image.get_pixel(1, 1), &Rgba([0, 255, 0, 255]));
        assert_eq!(image.get_pixel(2, 2), &Rgba([0, 255, 0, 255]));
        assert_eq!(image.get_pixel(99, 99), &Rgba([0, 255, 0, 255]));
        display_images!(2, image);
    }

    #[test]
    fn test_combination() {
        let mut image = ImageBuffer::new(100, 100);
        // 同时绘制多条直线
        line(0, 0, 100, 0, &mut image, Rgba([255, 0, 0, 255]));
        line(0, 0, 0, 100, &mut image, Rgba([0, 255, 0, 255]));
        line(10, 10, 10, 10, &mut image, Rgba([0, 0, 255, 255])); // 起点终点相同
        line(200, 200, 200, 200, &mut image, Rgba([255, 255, 0, 255])); // 超出范围
        display_images!(2, image);
    }

    // #[test]
    // fn test_performance() {
    //     let mut image = ImageBuffer::new(5000, 5000);
    //     for _ in 0..5000 {
    //         line(/*...*/, &mut image, /*...*/);
    //     }
    // }

    #[test]
    fn test_boundary_line() {
        let mut image = ImageBuffer::new(100, 100);
        line(0, 0, 100, 0, &mut image, Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(0, 0), &Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(50, 0), &Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(100, 0), &Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_line() {
        let mut image = ImageBuffer::new(100, 100);

        line(10, 10, 50, 50, &mut image, Rgba([255, 255, 255, 255])); // 使用白色绘制
        assert_eq!(image.get_pixel(10, 10), &Rgba([255, 255, 255, 255]));
        assert_eq!(image.get_pixel(50, 50), &Rgba([255, 255, 255, 255]));
        assert_eq!(image.get_pixel(30, 30), &Rgba([255, 255, 255, 255]));

        line(20, 13, 40, 80, &mut image, Rgba([255, 0, 0, 255])); // 使用红色绘制
        assert_eq!(image.get_pixel(50, 50), &Rgba([255, 0, 0, 255])); // 验证中间点的颜色是否为红色

        line(80, 40, 13, 20, &mut image, Rgba([255, 0, 0, 255])); // 使用红色绘制
        assert_eq!(image.get_pixel(90, 90), &Rgba([255, 0, 0, 255])); // 验证终点的颜色是否为红色
    }
}
