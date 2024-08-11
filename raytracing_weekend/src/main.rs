use std::rc::Rc;

use types::{objects::Sphere, Color, Hittable, HittableList, Point3, Ray, Vec3};

mod types;

const IMG_WIDTH: i32 = 400;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_HEIGHT: i32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as i32;

fn main() {
    let img_width = IMG_WIDTH;
    let img_height = if IMG_HEIGHT < 1 { 1 } else { IMG_HEIGHT };
    let aspect_ratio = img_width as f64 / img_height as f64;

    let focal_len = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let cam_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_du = viewport_u / img_width as f64;
    let pixel_dv = viewport_v / img_height as f64;

    let viewport_ul =
        cam_center + Vec3::new(0.0, 0.0, -focal_len) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_ul + pixel_du / 2.0 + pixel_dv / 2.0;

    let world = HittableList::from(vec![
        Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)) as Rc<dyn Hittable>,
        Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)) as Rc<dyn Hittable>,
    ]);

    println!("P3\n{img_width} {img_height}\n255");

    for j in 0..img_height {
        // eprint!("\r{} remaining", height - j);
        for i in 0..img_width {
            let pixel_center = pixel00_loc + pixel_dv * j + pixel_du * i;
            // eprintln!("{pixel_center} {} {}", pixel_dv * j, pixel_du * i);
            let ray_dir = pixel_center - cam_center;
            let ray = Ray::new(cam_center, ray_dir);
            let col = ray_color(&ray, &world);
            write_col(&col);
        }
    }
    eprintln!("\rDone!                ");
}

fn write_col(col: &Color) {
    assert!((0.0..=1.0).contains(&col.0), "0: {}", col.0);
    assert!((0.0..=1.0).contains(&col.1), "1: {}", col.1);
    assert!((0.0..=1.0).contains(&col.2), "2: {}", col.2);
    let r = (col.r() * 255.99) as i32;
    let g = (col.g() * 255.99) as i32;
    let b = (col.b() * 255.99) as i32;
    println!("{r} {g} {b}");
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    if let Some(rec) = world.hit(ray, 0.0..=f64::INFINITY) {
        // return Color::new(1.0, 0.0, 0.0);
        return (rec.normal() + Color::new(1.0, 1.0, 1.0)) / 2.0;
    }

    let unit_dir = ray.direction().to_unit_vec();
    // eprintln!("{} y={}", ray.direction(), unit_dir.y());
    let a = (unit_dir.y() + 1.0) / 2.0;
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}
