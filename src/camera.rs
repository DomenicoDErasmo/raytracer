use crate::{
    ray::Ray, 
    hittable::Hittable, 
    color::{Color, write_color}, 
    hit_record::HitRecord, 
    interval::Interval, 
    logger::{Logger, log},
    vec::{Point3, Vec3, Vec2, random_in_unit_disk}, util::{random_double, degrees_to_radians},
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image: Vec2<i32>,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vertical_field_of_view: f32,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vertical_up: Vec3,
    pub defocus_angle: f32,
    pub focus_distance: f32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta: Vec2<Vec3>,
    basis_u: Vec3,
    basis_v: Vec3,
    basis_w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image: Vec2 {width: 100, height: 0},
            samples_per_pixel: 10,
            max_depth: 10,
            vertical_field_of_view: 90.0,
            look_from: Point3 {x: 0.0, y: 0.0, z: -1.0},
            look_at: Point3::default(),
            vertical_up: Vec3 {x: 0.0, y: 1.0, z: 0.0},
            defocus_angle: f32::default(),
            focus_distance: 10.0,
            center: Point3::default(),
            pixel00_loc: Vec3::default(),
            pixel_delta: Vec2::default(),
            basis_u: Vec3::default(),
            basis_v: Vec3::default(),
            basis_w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, logger: &mut Logger, world: &mut impl Hittable) {
        self.initialize();

        log(
            &mut logger.stdout, 
            format!("P3\n{} {}\n255\n", self.image.width, self.image.height)
        );
        
        for j in 0..self.image.width {
            log(
                &mut logger.stderr, 
                format!("\rScanlines remaining: {}", self.image.width - j),
            );
            for i in 0..self.image.width {
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, self.max_depth, world);
                }
                write_color(&mut logger.stdout, pixel_color, self.samples_per_pixel);
            }
        }
    }

    fn initialize(&mut self) {
        // Image
        self.aspect_ratio = 16.0 / 9.0;
        self.image.width = 400;
        self.samples_per_pixel = 100;
        self.image.height = ((self.image.width as f32 / self.aspect_ratio) as i32).max(1);

        self.center = self.look_from;

        // Determine viewport dimensions
        let theta = degrees_to_radians(self.vertical_field_of_view);
        let h: f32 = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width = viewport_height 
            * self.image.width as f32 
            / self.image.height as f32;

        // Calcualte the u,v,w unit basis vectors for the camera coordinate frame.
        self.basis_w = (self.look_from - self.look_at).unit_vector();
        self.basis_u = self.vertical_up.cross(&self.basis_w).unit_vector();
        self.basis_v = self.basis_w.cross(&self.basis_u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport = Vec2 {
            width: viewport_width * self.basis_u,
            height: viewport_height * -self.basis_v,
        };

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta.width = viewport.width / self.image.width as _;
        self.pixel_delta.height = viewport.height / self.image.height as _;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = self.center 
            - (self.focus_distance * self.basis_w)
            - viewport.width / 2.0
            - viewport.height / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 
            * (self.pixel_delta.width + self.pixel_delta.height);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = self.focus_distance 
            * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.basis_u * defocus_radius;
        self.defocus_disk_v = self.basis_v * defocus_radius;
    }
    
    fn ray_color(&mut self, ray: &Ray, depth: i32, world: &mut impl Hittable) -> Color {
        let mut hit_record = HitRecord::default();

        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {return Color {x: 0.0, y: 0.0, z: 0.0};}

        if world.hit(ray, Interval {min: 0.001, max: f32::INFINITY}, &mut hit_record) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if hit_record.material.scatter(
                &ray, 
                &hit_record, 
                &mut attenuation, 
                &mut scattered
            ) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }
            return Color {x: 0.0, y: 0.0, z: 0.0};
        }
    
        let unit_direction = ray.direction.unit_vector();
        let alpha = 0.5 * (unit_direction.y + 1.0);
        (1.0 - alpha) * Color {x: 1.0, y: 1.0, z: 1.0} + alpha * Color{x: 0.5, y: 0.7, z: 1.0}
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc 
            + i as f32 * self.pixel_delta.width 
            + j as f32 * self.pixel_delta.height;
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        Ray {origin, direction: pixel_sample - origin}
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double(None, None);
        let py = -0.5 + random_double(None, None);
        px * self.pixel_delta.width + py * self.pixel_delta.height
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }
}