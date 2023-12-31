use image::GenericImageView;

use minifb::{Key, Window, WindowOptions};

/// 该示例代码将image 图像转为 [u32] 像素然后在窗口中进行了绘制
/// 但是这个代码有个问题就是RGBA通道值有点不对
/// 然后我们根据这个学习代码实现了render::display 中的 函数
fn main() {
    // 此处由于宏展开，必须使用编译器确定的变量才能正确将字节加载到二进制中
    let image_content = include_bytes!("../../resource/study/img.png");

    // 将字节数据加载为图像对象
    let image = image::load_from_memory(image_content).expect("Failed to load image");

    // 获取图像的宽度和高度
    let width = image.width() as usize;
    let height = image.height() as usize;

    // 创建窗口
    let mut window = Window::new("Image Viewer", width, height, WindowOptions::default())
        .expect("Failed to create window");

    // 转换图像数据为 RGBA 格式
    let rgba_image = image.to_rgba8();

    // 将图像像素值转换为u32格式
    let u32_image: Vec<u32> = rgba_image
        .pixels()
        .map(|pixel| {
            let [r, g, b, a] = pixel.0;
            // 通道顺序为RGBA，即先红色（R），然后绿色（G），然后蓝色（B），最后透明度（A）。
            ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
        })
        .collect();

    // 将图像数据传递给窗口进行显示
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&u32_image, width, height)
            .expect("Failed to update window");
    }
}
