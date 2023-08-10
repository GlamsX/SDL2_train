use sdl2::{rect::Point, render::Canvas};

use self::{point::Point3D, face::Face};
mod point;
mod face;
pub struct Object3d {
  pub faces: Vec<Face>,
}

impl Object3d {
  pub fn create_cube() -> Self {

    let triangles: Vec<Face> = vec![
        // Face avant
        Face { p1: Point3D::new(-1.0,-1.0,-1.0), p2: Point3D::new(1.0,-1.0,-1.0), p3: Point3D::new(1.0,1.0,-1.0)},
        Face { p1: Point3D::new(-1.0,-1.0,-1.0), p2: Point3D::new(1.0,1.0,-1.0), p3: Point3D::new(-1.0,1.0,-1.0)},
        // Face arrière
        Face { p1: Point3D::new(-1.0,-1.0,1.0), p2: Point3D::new(1.0,-1.0, 1.0), p3: Point3D::new(1.0,1.0,1.0)},
        Face { p1: Point3D::new(-1.0,-1.0,1.0), p2: Point3D::new(1.0,1.0,1.0), p3: Point3D::new(-1.0,1.0,1.0)},
        // Face de droite
        Face { p1: Point3D::new(1.0,-1.0,-1.0), p2: Point3D::new(1.0,-1.0,1.0), p3: Point3D::new(1.0,1.0,1.0)},
        Face { p1: Point3D::new(1.0,-1.0,-1.0), p2: Point3D::new(1.0,1.0,1.0), p3: Point3D::new(1.0,1.0,-1.0)},
        // Face de gauche
        Face { p1: Point3D::new(-1.0,-1.0,-1.0), p2: Point3D::new(-1.0,-1.0,1.0), p3: Point3D::new(-1.0,1.0,1.0)},
        Face { p1: Point3D::new(-1.0,-1.0,-1.0), p2: Point3D::new(-1.0,1.0,1.0), p3: Point3D::new(-1.0,1.0,-1.0 )},
        // Face supérieure
        Face { p1: Point3D::new(-1.0,1.0,-1.0), p2: Point3D::new(1.0,1.0,-1.0), p3: Point3D::new(1.0,1.0,1.0)},
        Face { p1: Point3D::new(-1.0,1.0,-1.0), p2: Point3D::new(1.0,1.0,1.0), p3: Point3D::new(-1.0,1.0,1.0)},
        // Face inférieure
        Face { p1: Point3D::new(-1.0,-1.0,-1.0), p2: Point3D::new(1.0,-1.0,-1.0), p3: Point3D::new(1.0,-1.0,1.0)},
        Face { p1: Point3D::new(-1.0,-1.0,-1.0), p2: Point3D::new(1.0,-1.0,1.0), p3: Point3D::new(-1.0,-1.0,1.0)},
    ];

    Object3d { faces: triangles }
  }
}