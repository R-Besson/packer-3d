use crate::box3d::Box3D;

use std::cmp::Ordering;
use Ordering::{
    Greater,
    Less,
    Equal,
};

pub struct Sorting {}

impl Sorting {
    pub fn descending_volume(a: &Box3D, b: &Box3D) -> Ordering // return 1 if b is before a (switch a and b), -1 otherwise
    {
        let bv = b.volume();
        let av = a.volume();

        if bv > av { return Greater; }
        if bv != av { return Less; }

        if b.size.z > a.size.z { return Greater; }
        if b.size.z != a.size.z { return Less; }

        if b.size.x > a.size.x { return Greater; }
        if b.size.x != a.size.x { return Less; }

        if b.size.y > a.size.y { return Greater; }
        if b.size.y != a.size.y { return Less; }

        if b.id < a.id { return Greater; }

        Equal
    }
    pub fn ascending_volume(a: &Box3D, b: &Box3D) -> Ordering
    {
        Self::descending_volume(b, a)
    }

    pub fn descending_width(a: &Box3D, b: &Box3D) -> Ordering
    {
        if b.size.x > a.size.x { return Greater; }
        if b.size.x != a.size.x { return Less; }

        let bv = b.volume();
        let av = a.volume();

        if bv > av { return Greater; }
        if bv != av { return Less; }

        if b.size.z > a.size.z { return Greater; }
        if b.size.z != a.size.z { return Less; }
        
        if b.size.y > a.size.y { return Greater; }
        if b.size.y != a.size.y { return Less; }

        if b.id < a.id { return Greater; }

        Equal
    }
    pub fn ascending_width(a: &Box3D, b: &Box3D) -> Ordering
    {
        Self::descending_width(b, a)
    }

    pub fn descending_height(a: &Box3D, b: &Box3D) -> Ordering
    {
        if b.size.y > a.size.y { return Greater; }
        if b.size.y != a.size.y { return Less; }
        
        let bv = b.volume();
        let av = a.volume();

        if bv > av { return Greater; }
        if bv != av { return Less; }

        if b.size.z > a.size.z { return Greater; }
        if b.size.z != a.size.z { return Less; }
        
        if b.size.x > a.size.x { return Greater; }
        if b.size.x != a.size.x { return Less; }

        if b.id < a.id { return Greater; }

        Equal
    }
    pub fn ascending_height(a: &Box3D, b: &Box3D) -> Ordering
    {
        Self::descending_height(b, a)
    }

    pub fn descending_length(a: &Box3D, b: &Box3D) -> Ordering
    {
        if b.size.z > a.size.z { return Greater; }
        if b.size.z != a.size.z { return Less; }
        
        let bv = b.volume();
        let av = a.volume();
        
        if bv > av { return Greater; }
        if bv != av { return Less; }

        if b.size.x > a.size.x { return Greater; }
        if b.size.x != a.size.x { return Less; }
        
        if b.size.y > a.size.y { return Greater; }
        if b.size.y != a.size.y { return Less; }

        if b.id < a.id { return Greater; }

        Equal
    }
    pub fn ascending_length(a: &Box3D, b: &Box3D) -> Ordering
    {
        Self::descending_length(b, a)
    }

    pub fn descending_id(a: &Box3D, b: &Box3D) -> Ordering
    {
        if b.id < a.id { return Greater; }

        Equal
    }
    pub fn ascending_id(a: &Box3D, b: &Box3D) -> Ordering
    {
        Self::descending_id(b, a)
    }
}