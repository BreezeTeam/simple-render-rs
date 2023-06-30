use image::{GenericImage, GenericImageView, ImageBuffer, Rgba};
use render::{display::*, display_images, geometry::line, geometry::point};
use std::sync::{atomic::AtomicBool, Arc, Mutex};

/**
 * 扫描线光栅化算法
 * 首先，确定三角形的顶点坐标（x1, y1），（x2, y2），（x3, y3）。
 * 找到三角形的最上边和最下边的扫描线，即确定扫描线的起始和结束y坐标范围。
 * 对于每一条扫描线，计算它与三角形边的交点。根据扫描线的y坐标和三角形的边的斜率，使用插值计算得到扫描线与边的交点的x坐标。
 * 根据得到的交点，确定每条扫描线的起始和结束x坐标。
 * 在每条扫描线的起始和结束x坐标之间填充像素
 */
fn triangle(
    v0: [i32; 3],
    v1: [i32; 3],
    v2: [i32; 3],
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    color: Rgba<u8>,
) {
    let mut v0 = v0;
    let mut v1 = v1;
    let mut v2 = v2;
    // 1.对三角形的顶点按照 y 坐标进行排序，确保顶点按照从上到下的顺序排列。
    if v0[1] > v1[1] {
        std::mem::swap(&mut v0, &mut v1);
    }
    if v0[1] > v2[1] {
        std::mem::swap(&mut v0, &mut v2);
    }
    if v1[1] > v2[1] {
        std::mem::swap(&mut v1, &mut v2);
    }

    // 2. 使用线段绘制算法栅格化三角形的左边和右边。
    // 根据高度插值得到，对于每一条水平扫描线，找到与这条扫描线相交的左边界点和右边界点。
    // 可以使用v1作为分割，然后上部分和下部分分别设置像素点

    //计算完整的三角形高度
    let total_height = v2[1] - v0[1] + 1;
    for h in 1..=total_height {
        // 判断当前是上一部分还是下一部分
        let second_half = h > v1[1] - v0[1] || v1[1] == v0[1];
        // segment_height 是当前扫描线所在段的高度
        let segment_height = if second_half {
            v2[1] - v1[1] + 1
        } else {
            v1[1] - v0[1] + 1
        };
        // 计算当前高度
        let alpha = h as f32 / total_height as f32;
        // beta 表示当前扫描线所在部分高度上的位置比例
        let beta = (h - if second_half { v1[1] - v0[1] } else { 0 }) as f32 / segment_height as f32;
        // 根据 alpha 和 beta 计算出当前扫描线的左边界点和右边界点
        let mut a: [f32; 2] = [0., 0.];
        let mut b: [f32; 2] = [0., 0.];
        for i in 0..2 {
            // 计算左边界点
            // 先计算向量v2->v0，然后乘以alpha 得到向量v2->v0的分量
            // 这样在x,y 坐标上就计算得到了坐标点
            a[i] = ((v2[i] - v0[i]) as f32 * alpha + v0[i] as f32) as f32;

            // 计算右边界点
            // 上半部分，那么就是v1->v0
            // 下半部分，那么就是v2->v1
            b[i] = if second_half {
                ((v2[i] - v1[i]) as f32 * beta + v1[i] as f32) as f32
            } else {
                ((v1[i] - v0[i]) as f32 * beta + v0[i] as f32) as f32
            };
        }
        // 确保 a 在 b 的左边
        if a[0] > b[0] {
            std::mem::swap(&mut a, &mut b);
        }
        // 从左边界点的x坐标开始，到右边界点的x结束，设置像素点
        for j in a[0] as i32..=b[0].round() as i32 {
            //坐标就是 x坐标:j 以及y坐标:h
            point(j, v0[1] + (h - 1), image, color)
        }
    }
}

/// 绘制三角形
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // 读取cargo.toml 文件所在地址
    let resource_path = format!("{}/../resource", env!("CARGO_MANIFEST_DIR"));
    log::info!("resource_path :{}", &resource_path);
    // Create a new ImageBuffer with width: imgx and height: imgy
    let mut original_image = ImageBuffer::new(500, 500);

    let red = Rgba([255, 0, 0, 255]); // 红色
    let white = Rgba([255, 255, 255, 255]); // 白色

    // 绘制三角形
    triangle(
        [10, 70, 1],
        [50, 160, 1],
        [70, 80, 1],
        &mut original_image,
        red,
    );

    //display
    original_image
        .save(&format!("{}/study/img2-triangle.png", resource_path))
        .expect("Failed to write clear TGA file");
    display_images!(5, original_image);
}
