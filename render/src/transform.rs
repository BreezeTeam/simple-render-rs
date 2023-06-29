use image::{DynamicImage, Rgba, RgbaImage};

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

    use image::Rgba;

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
