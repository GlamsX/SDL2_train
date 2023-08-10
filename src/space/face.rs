use sdl2::{render::Canvas, rect::Point};

use super::point::Point3D;

pub struct Face {
  pub p1: Point3D,
  pub p2: Point3D,
  pub p3: Point3D,
}

impl Face {
  pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>, width: f64, height: f64) {
    let (x1, y1) = self.p1.orthographic_projection_y(100.0, 100.0, width, height);
    let (x2, y2) = self.p2.orthographic_projection_y(100.0, 100.0, width, height);
    let (x3, y3) = self.p3.orthographic_projection_y(100.0, 100.0, width, height);

    canvas.draw_line(Point::new(x1 as i32, y1 as i32), Point::new(x2 as i32, y2 as i32)).unwrap();
    canvas.draw_line(Point::new(x2 as i32, y2 as i32), Point::new(x3 as i32, y3 as i32)).unwrap();
    canvas.draw_line(Point::new(x3 as i32, y3 as i32), Point::new(x1 as i32, y1 as i32)).unwrap();
  }
}