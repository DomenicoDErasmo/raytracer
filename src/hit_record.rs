use crate::{
    ray::Ray, 
    material::Material, 
    lambertian::Lambertian, 
    color::Color, 
    vec::{Point3, Vec3}, 
};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub time: f32,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Point3::default(),
            normal: Vec3::default(),
            material: Box::<Lambertian>::new(Lambertian { albedo: Color {x: 0.0, y: 0.0, z: 0.0}}),
            time: f32::default(),
            front_face: bool::default(),
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // outward_normal is assumed to have unit length

        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-*outward_normal};
    }
}