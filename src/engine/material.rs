use super::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{dot, random_unit_vector, reflect, refract, vec3},
};
use dyn_clone::DynClone;

pub trait Material: DynClone + Send + Sync {
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

dyn_clone::clone_trait_object!(Material);

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal() + random_unit_vector();

        if f32::abs(scatter_direction.x) < 1e-8
            && f32::abs(scatter_direction.y) < 1e-8
            && f32::abs(scatter_direction.z) < 1e-8
        {
            scatter_direction = rec.normal();
        }

        scattered.clone_from(&Ray::new(rec.p, scatter_direction.normalize()));
        attenuation.clone_from(&self.albedo);
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(r.direction().normalize(), rec.normal());
        scattered.clone_from(&Ray::new(
            rec.p,
            reflected.normalize() + self.fuzz * random_unit_vector(),
        ));
        attenuation.clone_from(&self.albedo);
        true
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub ir: f32,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        attenuation.clone_from(&Color::new(1.0, 1.0, 1.0));
        let mut reflection_ratio = self.ir;
        if rec.front_face {
            reflection_ratio = 1.0 / self.ir;
        }

        let unit_dir = r.direction().normalize();
        let cos0 = f32::min(dot(&-unit_dir, &rec.normal()), 1.0);
        let sin0 = f32::sqrt(1.0 - cos0 * cos0);

        let cannot_refract = reflection_ratio * sin0 > 1.0;
        let mut direction = vec3(0.0);

        if cannot_refract {
            direction = reflect(unit_dir, rec.normal());
        } else {
            direction = refract(unit_dir, rec.normal(), reflection_ratio);
        }
        scattered.clone_from(&Ray::new(rec.p, direction));
        true
    }
}
