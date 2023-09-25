use crate::{
    ray::Ray, 
    hittable::Hittable, 
    color::{Color, write_color}, 
    hit_record::HitRecord, 
    interval::Interval, 
    logger::{Logger, log},
    vec::{Point3, Vec3, Vec2},
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image: Vec2<i32>,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta: Vec2<Vec3>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image: Vec2 {width: 100, height: 0},
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta: Vec2::default(),
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
        
        for row in 0..self.image.width {
            log(
                &mut logger.stderr, 
                format!("\rScanlines remaining: {}", self.image.width - row),
            );
            for col in 0..self.image.width {
                let pixel_center = self.pixel00_loc 
                    + (col as f32 * self.pixel_delta.width) 
                    + (row as f32 * self.pixel_delta.height);
                let ray = Ray { origin: self.center, direction: pixel_center - self.center };
                let pixel_color = self.ray_color(&ray, world);
                write_color(&mut logger.stdout, pixel_color);
            }
        }
    }

    fn initialize(&mut self) {
        // Image
        self.aspect_ratio = 16.0 / 9.0;
        self.image.width = 400;
        self.image.height = ((self.image.width as f32 / self.aspect_ratio) as i32).max(1);

        // Camera
        let viewport_height = 2.0;
        let viewport_width = viewport_height 
            * self.image.width as f32 
            / self.image.height as f32;
        self.center = Point3 {x: 0.0, y: 0.0, z: 0.0};

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport = Vec2 {
            width: Vec3 {x: viewport_width, y: 0.0, z: 0.0},
            height: Vec3 {x: 0.0, y: -viewport_height, z: 0.0},
        };

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta.width = viewport.width / self.image.width as _;
        self.pixel_delta.height = viewport.height / self.image.height as _;

        // Calculate the location of the upper left pixel
        let focal_length = 1.0;
        let viewport_upper_left = self.center 
            - Vec3 {x: 0.0, y: 0.0, z: focal_length} 
            - viewport.width / 2.0 - viewport.height / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 
            * (self.pixel_delta.width + self.pixel_delta.height);
    }
    
    fn ray_color(&mut self, ray: &Ray, world: &mut impl Hittable) -> Color {
        let mut hit_record = HitRecord::default();
        if world.hit(ray, Interval {min: 0.0, max: f32::INFINITY}, &mut hit_record) {
            return 0.5 * (hit_record.normal + Color{x: 1.0, y: 1.0, z: 1.0});
        }
    
        let unit_direction = ray.direction.unit_vector();
        let alpha = 0.5 * (unit_direction.y + 1.0);
        (1.0 - alpha) * Color {x: 1.0, y: 1.0, z: 1.0} + alpha * Color{x: 0.5, y: 0.7, z: 1.0}
    }
}