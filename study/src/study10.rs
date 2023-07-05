use image::imageops::flip_vertical_in_place;
use image::{GenericImage, GenericImageView, ImageBuffer, Rgba};
use nalgebra::{Point3, Vector3};
use obj::{Obj, ObjData, Object, SimplePolygon};
use render::transform::flip_vertically;
use render::{display::*, display_images, geometry::*};
use std::fs::File;
use std::io::BufReader;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

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

/**
 * 片段光栅化算法（逐片段光栅化算法）
 * 确定三角形的顶点坐标（x1, y1），（x2, y2），（x3, y3）。
 * 找到三角形的最小包围盒（bounding box），即确定三角形在屏幕上的最小矩形范围。
 * 针对最小包围盒内的每个像素，使用重心坐标插值计算像素对应在三角形上的位置。
 * 判断计算得到的位置是否在三角形内部，如果在内部，则填充像素。
 * 重复步骤3和步骤4，直到遍历完最小包围盒内的所有像素。
 */
fn triangle_rasterization(
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

fn interpolate(min: f32, max: f32, grad: f32) -> f32 {
    min + (max - min) * grad
}

fn triangle(
    v0: [i32; 3],
    v1: [i32; 3],
    v2: [i32; 3],
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    color: Rgba<u8>,
) {
    triangle_rasterization(v0, v1, v2, color, color, color, image);
}

/// 将顶点转换其xy坐标到图片坐标
fn vec_trans(width: u32, height: u32, vertices: Vec<[f32; 3]>) -> Vec<[i32; 3]> {
    let mut transformed_vertices: Vec<[i32; 3]> = Vec::new();
    for vertex in vertices.iter() {
        let x = (vertex[0] + 1.) as f32 * width as f32 / 2.0;
        let y = (vertex[1] + 1.) as f32 * height as f32 / 2.0;
        let transformed_vertex: [i32; 3] = [x as i32, y as i32, vertex[2] as i32];
        transformed_vertices.push(transformed_vertex);
    }
    return transformed_vertices;
}

/// 获取obj文件中的所有的polygon，以顶点坐标列表的形式返回
fn get_obj_polygons(obj_data: ObjData) -> Vec<Vec<[f32; 3]>> {
    // 存储这个obj中所有polygon的所有顶点索引
    let mut faces = Vec::new();
    // 获取顶点坐标
    let position_data = obj_data.position;
    // 遍历绘制全部的对象的polygon
    for object in obj_data.objects {
        for group in object.groups {
            for idx_vec in group.polys {
                if let SimplePolygon(polygon) = idx_vec {
                    let mut vertices = Vec::<[f32; 3]>::new();
                    polygon.iter().for_each(|ids| {
                        // 获取顶点坐标
                        let position = position_data[ids.0];
                        vertices.push(position);
                    });
                    faces.push(vertices.clone());
                }
            }
        }
    }
    return faces;
}

/// 读取 解析 obj https://github.com/simnalamburt/obj-rs
/// 直接绘制obj 里顶点连接的线
/// 忽略顶点的z坐标
/// https://zhuanlan.zhihu.com/p/149836719
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // 读取cargo.toml 文件所在地址
    let resource_path = format!("{}/../resource", env!("CARGO_MANIFEST_DIR"));
    log::info!("resource_path :{}", &resource_path);

    // 创建一个imgbuf
    let width = 800;
    let height = 800;
    let mut original_image = ImageBuffer::new(width, height);

    // 第一种加载方式
    let obj_object = Obj::load(&format!("{}/obj/african_head.obj", resource_path))
        .expect("Failed to read OBJ file");
    let obj_data = obj_object.data;

    // 第二种加载方式
    // 将文件转为bytes Reader 接口
    let obj_content = include_bytes!("../../resource/study/img.png");
    let obj_content = BufReader::new(
        File::open(&format!("{}/obj/african_head.obj", resource_path))
            .expect("Failed to read OBJ file"),
    );
    // 加载对象
    let obj_data = ObjData::load_buf(obj_content).expect("Failed to load OBJ file");

    // 获取polygons
    for polygon in get_obj_polygons(obj_data) {
        let transformed_vertices = vec_trans(width, height, polygon);

        // 根据polygon转换的到的像素坐标，进行线段绘制
        // 我们只绘制xy坐标，z坐标是深度信息，暂时忽略
        for i in 0..2 {
            let v0 = transformed_vertices[i];
            let v1 = transformed_vertices[i + 1];
            line(
                v0[0],
                v0[1],
                v1[0],
                v1[1],
                &mut original_image,
                Rgba([255, 0, 0, 255]),
            );
        }
        triangle(
            transformed_vertices[0],
            transformed_vertices[1],
            transformed_vertices[2],
            &mut original_image,
            random_rgba(),
        );
    }

    flip_vertically(&mut original_image);
    original_image
        .save(&format!("{}/study/img2-faces.png", resource_path))
        .expect("Failed to write clear TGA file");
    display_images!(60, original_image);
}
