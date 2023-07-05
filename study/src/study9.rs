use image::imageops::flip_vertical_in_place;
use image::{GenericImage, GenericImageView, ImageBuffer, Rgba};
use nalgebra::{Point3, Vector3};
use obj::{Obj, ObjData, Object, SimplePolygon};
use render::transform::flip_vertically;
use render::{display::*, display_images, geometry::*};
use std::fs::File;
use std::io::BufReader;
use std::sync::{atomic::AtomicBool, Arc, Mutex};
use std::vec;
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

/// 将顶点转换其xy坐标到图片坐标
fn vec_trans(width: u32, height: u32, vertices: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
    let mut transformed_vertices: Vec<[f32; 3]> = Vec::new();
    for vertex in vertices.iter() {
        let x = (vertex[0] + 1.) as f32 * width as f32 / 2.0;
        let y = (vertex[1] + 1.) as f32 * height as f32 / 2.0;
        let transformed_vertex: [f32; 3] = [x as f32, y as f32, vertex[2] as f32];
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

/// 计算三角形的法向量
fn calculate_normal(vertices: &[Point3<f32>; 3]) -> Vector3<f32> {
    let edge1 = vertices[1] - vertices[0];
    let edge2 = vertices[2] - vertices[0];
    edge1.cross(&edge2)
}

/// 读取 解析 obj https://github.com/simnalamburt/obj-rs
/// 直接绘制obj 里顶点连接的线
/// 忽略顶点的z坐标
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
    let light_dir = Vector3::new(0.0, 0.0, -1.0);
    // 获取polygons
    for polygon in get_obj_polygons(obj_data) {
        let transformed_vertices = vec_trans(width, height, polygon);

        // // 根据polygon转换的到的像素坐标，进行线段绘制
        // // 我们只绘制xy坐标，z坐标是深度信息，暂时忽略
        // for i in 0..2 {
        //     let v0 = transformed_vertices[i];
        //     let v1 = transformed_vertices[i + 1];
        //     line(
        //         v0[0],
        //         v0[1],
        //         v1[0],
        //         v1[1],
        //         &mut original_image,
        //         Rgba([255, 0, 0, 255]),
        //     );
        // }

        // 获得变换后的三角形顶点
        let vertex1 = Point3::new(transformed_vertices[0][0] as f32, transformed_vertices[0][1] as f32, transformed_vertices[0][2] as f32);
        let vertex2 = Point3::new(transformed_vertices[1][0] as f32, transformed_vertices[1][1] as f32, transformed_vertices[1][2] as f32);
        let vertex3 = Point3::new(transformed_vertices[2][0] as f32, transformed_vertices[2][1] as f32, transformed_vertices[2][2] as f32);
        let normal:Vector3<f32> = calculate_normal(&[vertex1, vertex2, vertex3]);


        let mut intensity = normal.dot(&light_dir);
        log::info!( "intensity :{:?}", intensity);

        if intensity > 0. {
            let color = Rgba([intensity as u8, intensity as u8, intensity as u8, 255]);
            triangle(
                transformed_vertices[0].map(|x| x as i32),
                transformed_vertices[1].map(|x| x as i32),
                transformed_vertices[2].map(|x| x as i32),
                &mut original_image,
                color
            );
        }

    }

    flip_vertically(&mut original_image);
    original_image
        .save(&format!("{}/study/img2-faces.png", resource_path))
        .expect("Failed to write clear TGA file");
    // display_images!(60, original_image);
}
