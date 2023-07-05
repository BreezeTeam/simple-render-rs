use image::imageops::flip_vertical_in_place;
use image::{GenericImage, GenericImageView, ImageBuffer, Rgba};
use nalgebra::{Point3, Vector3};
use obj::{Obj, ObjData, Object, SimplePolygon};
use render::transform::flip_vertically;
use render::{display::*, display_images, geometry::*};
use std::fs::File;
use std::io::BufReader;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

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

/// 计算三角形的法向量
fn calculate_normal(vertices: &[Point3<f32>; 3]) -> Vector3<f32> {
    let edge1 = vertices[2] - vertices[0];
    let edge2 = vertices[1] - vertices[0];
    edge1.cross(&edge2).normalize()
}

/// 将三角面的颜色改成光照信息
/// https://zhuanlan.zhihu.com/p/402026577
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
    // 光照方向
    let light_dir = Vector3::new(0.0, 0.0, -1.0);
    // 获取polygons
    for polygon in get_obj_polygons(obj_data) {
        let transformed_vertices = vec_trans(width, height, polygon.clone());

        // 获得变换后的三角形顶点
        let point3_array: [Point3<f32>; 3] = polygon
            .iter()
            .map(|arr| Point3::new(arr[0], arr[1], arr[2]))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        // 计算三角面的法向量
        let normal: Vector3<f32> = calculate_normal(&point3_array);

        // 计算方向光照强度
        let mut intensity = normal.dot(&light_dir);
        // 背面剔除,减少绘制次数
        if intensity > 0. {
            intensity = intensity * 255.0;
            let color = Rgba([intensity as u8, intensity as u8, intensity as u8, 255]);
            triangle(
                transformed_vertices[0],
                transformed_vertices[1],
                transformed_vertices[2],
                &mut original_image,
                color
            );
            // triangle_rasterization(
            //     transformed_vertices[0],
            //     transformed_vertices[1],
            //     transformed_vertices[2],
            //     color,
            //     color,
            //     color,
            //     &mut original_image,
            // );
        }
    }

    flip_vertically(&mut original_image);
    original_image
        .save(&format!("{}/study/img2-faces.png", resource_path))
        .expect("Failed to write clear TGA file");
    display_images!(10, original_image);
}
