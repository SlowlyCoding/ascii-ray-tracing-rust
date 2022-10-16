pub mod sphere;
pub mod triangle;
pub mod cube;
use crate::ray;
use crate::scene;

pub trait Object {
    fn intersection(&self, ray: &ray::Ray, ii: &mut scene::IntersectionInformation) -> bool;
}
