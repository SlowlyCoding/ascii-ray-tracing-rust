pub mod sphere;
pub mod triangle;
use crate::ray;
use crate::scene;

pub trait Object {
    fn intersection(&self, ray: &ray::Ray, ii: &mut scene::IntersectionInformation) -> bool;
}
