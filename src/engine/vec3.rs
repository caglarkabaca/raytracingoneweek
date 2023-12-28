use glam::f32::Vec3;
use rand::Rng;

pub fn vec3(x: f32) -> Vec3 {
    Vec3::new(x, x, x)
}

pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn random_vec3() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}

pub fn random_vec3_range(min: f32, max: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, &normal) > 0.0 {
        return on_unit_sphere;
    }
    return -on_unit_sphere;
}
