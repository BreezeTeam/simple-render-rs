use image::imageops::flip_vertical_in_place;
use image::{GenericImage, GenericImageView, ImageBuffer, Rgba};
use obj::{Obj, ObjData, Object, SimplePolygon};
use render::transform::flip_vertically;
use render::{display::*, display_images, geometry::*};
use std::fs::File;
use std::io::BufReader;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

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

    // 获取数据！
    // 获取顶点坐标
    let position_data = obj_data.position;
    // 获取纹理坐标
    let texture_data = obj_data.texture;
    // 获取法向量
    let normal_data = obj_data.normal;

    // 存储这个obj中所有polygon的所有顶点索引
    let mut faces = Vec::new();

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
                        // 获取纹理坐标
                        let texture = texture_data[ids.1.unwrap()];
                        // 获取法向量
                        let normal = normal_data[ids.2.unwrap()];
                        log::debug!(
                            "obj_name :{:?} group.name:{:?} position :{:?} texture:{:?} normal:{:?}",
                            object.name,
                            group.name,
                            position,
                            texture,
                            normal
                        );
                    });
                    faces.push(vertices.clone());
                    log::debug!(
                        "obj_name :{:?} group.name:{:?} vertices :{:?}",
                        object.name,
                        group.name,
                        vertices.clone(),
                    );

                    // for v in vertices {
                    //     // 计算屏幕坐标
                    //     let x0 = (v[0] / v[2]) * fov_x + center_x;
                    //     let y0 = (v[1] / v[2]) * fov_y + center_y;
                    // 绘制点
                    // set_pixel(x0, y0, &mut image, white);
                    // }

                    for i in 0..2 {
                        let v0 = vertices[i];
                        let v1 = vertices[i + 1];

                        let x0 = (v0[0] + 1.) * width as f32 / 2.;
                        let y0 = (v0[1] + 1.) * height as f32 / 2.;
                        let x1 = (v1[0] + 1.) * width as f32 / 2.;
                        let y1 = (v1[1] + 1.) * height as f32 / 2.;

                        // let x0 = (v0[0] / v0[2]) * fov_x + center_x;
                        // let y0 = (v0[1] / v0[2]) * fov_y + center_y;
                        // let x1 = (v1[0] / v1[2]) * fov_x + center_x;
                        // let y1 = (v1[1] / v1[2]) * fov_y + center_y;

                        line(
                            x0 as i32,
                            y0 as i32,
                            x1 as i32,
                            y1 as i32,
                            &mut original_image,
                            Rgba([255, 0, 0, 255]),
                        );
                    }
                }
            }
        }
    }

    log::info!("faces :{:?} ", faces.len());
    flip_vertically(&mut original_image);
    original_image
        .save(&format!("{}/study/img-obj-polygons.png", resource_path))
        .expect("Failed to write clear TGA file");
    display_images!(5, original_image);
}
