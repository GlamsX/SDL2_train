use sdl2::{rect::Point, render::Canvas};

pub struct Point3D {
  x: f64,
  y: f64,
  z: f64,
}

pub struct Face {
  pub p1: Point3D,
  pub p2: Point3D,
  pub p3: Point3D,
}
pub struct Object3d {
  pub faces: Vec<Face>,
}

impl Point3D {
  pub fn project(&self, width: f64, height: f64) -> (f64, f64) {
    let scale = 200.0 / self.z;
    let x = self.x * scale + width / 2.0;
    let y = -self.y * scale + height / 2.0;
    (x, y)
}

  pub fn translate(&mut self, x: f64, y: f64, z: f64) {
    self.x += x;
    self.y += y;
    self.z += z;
  }

  pub fn scale(&mut self, factor: f64) {
    self.x *= factor;
    self.y *= factor;
    self.z *= factor;
  }

  pub fn rotate_x(&mut self, angle: f64) {
    let y = self.y;
    self.y = y * angle.cos() - self.z * angle.sin();
    self.z = y * angle.sin() + self.z * angle.cos();
  }

  pub fn rotate_y(&mut self, angle: f64) {
    let x = self.x;
    self.x = x * angle.cos() - self.z * angle.sin();
    self.z = x * angle.sin() + self.z * angle.cos();
  }

  pub fn rotate_z(&mut self, angle: f64) {
    let x = self.x;
    self.x = x * angle.cos() - self.y * angle.sin();
    self.y = x * angle.sin() + self.y * angle.cos();
  }

}

impl Face {
  pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>, width: f64, height: f64) {
    let (x1, y1) = self.p1.project(width, height);
    let (x2, y2) = self.p2.project(width, height);
    let (x3, y3) = self.p3.project(width, height);

    canvas.draw_line(Point::new(x1 as i32, y1 as i32), Point::new(x2 as i32, y2 as i32)).unwrap();
    canvas.draw_line(Point::new(x2 as i32, y2 as i32), Point::new(x3 as i32, y3 as i32)).unwrap();
    canvas.draw_line(Point::new(x3 as i32, y3 as i32), Point::new(x1 as i32, y1 as i32)).unwrap();
  }
}

impl Object3d {
  pub fn create_cube() -> Self {

    let triangles: Vec<Face> = vec![
        // Face avant
        Face { p1: Point3D { x: -1.0, y: -1.0, z: -1.0 }, p2: Point3D { x:  1.0, y: -1.0, z: -1.0 }, p3: Point3D { x:  1.0, y:  1.0, z: -1.0 } },
        Face { p1: Point3D { x: -1.0, y: -1.0, z: -1.0 }, p2: Point3D { x:  1.0, y:  1.0, z: -1.0 }, p3: Point3D { x: -1.0, y:  1.0, z: -1.0 } },
        // Face arrière
        Face { p1: Point3D { x: -1.0, y: -1.0, z:  1.0 }, p2: Point3D { x:  1.0, y: -1.0, z:  1.0 }, p3: Point3D { x:  1.0, y:  1.0, z:  1.0 } },
        Face { p1: Point3D { x: -1.0, y: -1.0, z:  1.0 }, p2: Point3D { x:  1.0, y:  1.0, z:  1.0 }, p3: Point3D { x: -1.0, y:  1.0, z:  1.0 } },
        // Face de droite
        Face { p1: Point3D { x:  1.0, y: -1.0, z: -1.0 }, p2: Point3D { x:  1.0, y: -1.0, z:  1.0 }, p3: Point3D { x:  1.0, y:  1.0, z:  1.0 } },
        Face { p1: Point3D { x:  1.0, y: -1.0, z: -1.0 }, p2: Point3D { x:  1.0, y:  1.0, z:  1.0 }, p3: Point3D { x:  1.0, y:  1.0, z: -1.0 } },
        // Face de gauche
        Face { p1: Point3D { x: -1.0, y: -1.0, z: -1.0 }, p2: Point3D { x: -1.0, y: -1.0, z:  1.0 }, p3: Point3D { x: -1.0, y:  1.0, z:  1.0 } },
        Face { p1: Point3D { x: -1.0, y: -1.0, z: -1.0 }, p2: Point3D { x: -1.0, y:  1.0, z:  1.0 }, p3: Point3D { x: -1.0, y:  1.0, z: -1.0 } },
        // Face supérieure
        Face { p1: Point3D { x: -1.0, y:  1.0, z: -1.0 }, p2: Point3D { x:  1.0, y:  1.0, z: -1.0 }, p3: Point3D { x:  1.0, y:  1.0, z:  1.0 } },
        Face { p1: Point3D { x: -1.0, y:  1.0, z: -1.0 }, p2: Point3D { x:  1.0, y:  1.0, z:  1.0 }, p3: Point3D { x: -1.0, y:  1.0, z:  1.0 } },
        // Face inférieure
        Face { p1: Point3D { x: -1.0, y: -1.0, z: -1.0 }, p2: Point3D { x:  1.0, y: -1.0, z: -1.0 }, p3: Point3D { x:  1.0, y: -1.0, z:  1.0 } },
        Face { p1: Point3D { x: -1.0, y: -1.0, z: -1.0 }, p2: Point3D { x:  1.0, y: -1.0, z:  1.0 }, p3: Point3D { x: -1.0, y: -1.0, z:  1.0 } },
    ];

    Object3d { faces: triangles }
  }
}