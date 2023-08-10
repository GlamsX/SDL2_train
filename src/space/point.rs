pub struct Point3D {
  x: f64,
  y: f64,
  z: f64,
}

impl Point3D {

  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Point3D {
      x,
      y,
      z,
    }
  }

  pub fn project(&self, width: f64, height: f64) -> (f64, f64) {
    let scale = 200.0 / self.z;
    let x = self.x * scale + width / 2.0;
    let y = -self.y * scale + height / 2.0;
    (x, y)
  }

  pub fn orthographic_projection_y(&self, sx: f64, sz: f64, cx: f64, cz: f64) -> (f64, f64) {
    let c_x = cx/2.0;
    let c_z = cz/2.0;
    let bx = sx * self.x + c_x;
    let by = sz * self.z + c_z;
    (bx, by)
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
