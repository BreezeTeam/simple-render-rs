use image::{ImageBuffer, Rgba};
use nalgebra::{Point3, Vector3};

/// 绘制一个像素点
/// 具有边界检查
pub fn point(x: i32, y: i32, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, color: Rgba<u8>) {
    if image.width() as i32 > x && x >= 0 && image.height() as i32 > y && y >= 0 {
        image.put_pixel(x as u32, y as u32, color);
    }
}

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

// 随机生成 RGBA
pub fn random_rgba() -> Rgba<u8> {
    let red: u8 = rand::random::<u8>();
    let green: u8 = rand::random::<u8>();
    let blue: u8 = rand::random::<u8>();
    let alpha: u8 = rand::random::<u8>();
    return Rgba([red, green, blue, alpha]);
}

/**
 * 扫描线光栅化算法
 * 首先，确定三角形的顶点坐标（x1, y1），（x2, y2），（x3, y3）。
 * 找到三角形的最上边和最下边的扫描线，即确定扫描线的起始和结束y坐标范围。
 * 对于每一条扫描线，计算它与三角形边的交点。根据扫描线的y坐标和三角形的边的斜率，使用插值计算得到扫描线与边的交点的x坐标。
 * 根据得到的交点，确定每条扫描线的起始和结束x坐标。
 * 在每条扫描线的起始和结束x坐标之间填充像素
 * TODO:need test
 */
pub fn triangle(
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

/**
 * 片段光栅化算法（逐片段光栅化算法）
 * 确定三角形的顶点坐标（x1, y1），（x2, y2），（x3, y3）。
 * 找到三角形的最小包围盒（bounding box），即确定三角形在屏幕上的最小矩形范围。
 * 针对最小包围盒内的每个像素，使用重心坐标插值计算像素对应在三角形上的位置。
 * 判断计算得到的位置是否在三角形内部，如果在内部，则填充像素。
 * 重复步骤3和步骤4，直到遍历完最小包围盒内的所有像素。
 */
pub fn triangle_rasterization(
    v0: [i32; 3],
    v1: [i32; 3],
    v2: [i32; 3],
    c0: Rgba<u8>,
    c1: Rgba<u8>,
    c2: Rgba<u8>,
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
) {
    // 获取bbox
    let min_x = i32::min(v0[0], i32::min(v1[0], v2[0]));
    let max_x = i32::max(v0[0], i32::max(v1[0], v2[0]));
    let min_y = i32::min(v0[1], i32::min(v1[1], v2[1]));
    let max_y = i32::max(v0[1], i32::max(v1[1], v2[1]));
    // 针对每个像素，计算器在三角形上的位置，判断其是否在三角形中
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // 获得变换后的三角形顶点
            let pts: [Point3<i32>; 3] = vec![v0, v1, v2]
                .iter()
                .map(|arr| Point3::new(arr[0], arr[1], arr[2]))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let p_w = barycentric(&pts, Point3::new(x, y, 0));
            if p_w.x >= 0. && p_w.y >= 0. && p_w.z >= 0. {
                let Rgba([r0, g0, b0, _]) = c0;
                let Rgba([r1, g1, b1, _]) = c1;

                let r = interpolate(r0 as f32, r1 as f32, p_w.x as f32) as u8;
                let g = interpolate(g0 as f32, g1 as f32, p_w.x as f32) as u8;
                let b = interpolate(b0 as f32, b1 as f32, p_w.x as f32) as u8;

                point(x, y, image, Rgba([r, g, b, 255]));
            }
        }
    }
}

//计算重心坐标
fn barycentric(pts: &[Point3<i32>; 3], p: Point3<i32>) -> Vector3<f32> {
    let u = Vector3::new(
        (pts[2].x - pts[0].x) as f32,
        (pts[1].x - pts[0].x) as f32,
        (pts[0].x - p.x) as f32,
    )
    .cross(&Vector3::new(
        (pts[2].y - pts[0].y) as f32,
        (pts[1].y - pts[0].y) as f32,
        (pts[0].y - p.y) as f32,
    ));

    // 如果三角形退化（面积接近于0），返回具有负坐标的结果
    if u.z.abs() < 1.0 {
        return Vector3::new(-1.0, 1.0, 1.0);
    }

    let w0 = 1.0 - (u.x + u.y) / u.z;
    let w1 = u.y / u.z;
    let w2 = u.x / u.z;

    Vector3::new(w0, w1, w2)
}

fn interpolate(min: f32, max: f32, grad: f32) -> f32 {
    min + (max - min) * grad
}

#[cfg(test)]
mod line_tests {
    use super::super::display_images;
    use super::*;
    use image::ImageBuffer;
    use rand::Rng;

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

        line(0, 0, 1000, 1000, &mut image, Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(0, 0), &Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(50, 50), &Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(99, 99), &Rgba([255, 0, 0, 255]));
        display_images!(2, image);
    }

    #[test]
    fn test_combination() {
        let mut image = ImageBuffer::new(100, 100);
        //绘制由左上到右上的
        line(10, 10, 90, 10, &mut image, Rgba([255, 255, 255, 255]));
        assert_eq!(image.get_pixel(10, 10), &Rgba([255, 255, 255, 255]));
        assert_eq!(image.get_pixel(50, 10), &Rgba([255, 255, 255, 255]));
        assert_eq!(image.get_pixel(90, 10), &Rgba([255, 255, 255, 255]));

        line(90, 10, 10, 10, &mut image, Rgba([0, 0, 255, 255]));
        assert_eq!(image.get_pixel(90, 10), &Rgba([0, 0, 255, 255]));
        assert_eq!(image.get_pixel(50, 10), &Rgba([0, 0, 255, 255]));
        assert_eq!(image.get_pixel(10, 10), &Rgba([0, 0, 255, 255]));

        //绘制由左上到右下的
        line(10, 10, 90, 90, &mut image, Rgba([255, 0, 255, 255]));
        assert_eq!(image.get_pixel(10, 10), &Rgba([255, 0, 255, 255]));
        assert_eq!(image.get_pixel(50, 50), &Rgba([255, 0, 255, 255]));
        assert_eq!(image.get_pixel(90, 90), &Rgba([255, 0, 255, 255]));

        line(90, 90, 10, 10, &mut image, Rgba([255, 255, 255, 255]));
        assert_eq!(image.get_pixel(10, 10), &Rgba([255, 255, 255, 255]));
        assert_eq!(image.get_pixel(50, 50), &Rgba([255, 255, 255, 255]));
        assert_eq!(image.get_pixel(90, 90), &Rgba([255, 255, 255, 255]));

        //绘制由左上到左下的
        line(10, 10, 10, 90, &mut image, Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(10, 10), &Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(10, 50), &Rgba([255, 0, 0, 255]));
        assert_eq!(image.get_pixel(10, 90), &Rgba([255, 0, 0, 255]));

        line(10, 90, 10, 10, &mut image, Rgba([255, 0, 255, 255]));
        assert_eq!(image.get_pixel(10, 10), &Rgba([255, 0, 255, 255]));
        assert_eq!(image.get_pixel(10, 50), &Rgba([255, 0, 255, 255]));
        assert_eq!(image.get_pixel(10, 90), &Rgba([255, 0, 255, 255]));

        //绘制由右上到左下的
        line(90, 10, 10, 90, &mut image, Rgba([255, 0, 255, 255]));
        assert_eq!(image.get_pixel(90, 10), &Rgba([255, 0, 255, 255]));
        assert_eq!(image.get_pixel(50, 50), &Rgba([255, 0, 255, 255]));
        assert_eq!(image.get_pixel(10, 90), &Rgba([255, 0, 255, 255]));

        line(10, 90, 90, 10, &mut image, Rgba([255, 255, 255, 255]));
        assert_eq!(image.get_pixel(10, 90), &Rgba([255, 255, 255, 255]));
        assert_eq!(image.get_pixel(50, 50), &Rgba([255, 255, 255, 255]));
        assert_eq!(image.get_pixel(90, 10), &Rgba([255, 255, 255, 255]));

        display_images!(2, image);
    }

    #[test]
    fn test_performance() {
        let width = 1920;
        let height = 1080;
        let mut image = ImageBuffer::new(width, height);

        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let rx1 = rng.gen_range(0..width) as i32;
            let ry1 = rng.gen_range(0..height) as i32;
            let rx2 = rng.gen_range(0..width) as i32;
            let ry2 = rng.gen_range(0..height) as i32;
            line(rx1, ry1, rx2, ry2, &mut image, random_rgba());
        }
        display_images!(2, image);
    }
}
