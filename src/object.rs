use crate::{
    hittable::Hittable,
    material::Material,
};

#[derive(Clone)]
pub struct Object {
    pub hittable: Hittable,
    pub material: Material,
}
