use std::ops::Mul;

/// matrix math library
/// note: all angles are in radians
pub enum Axis
{
  X,
  Y,
  Z,
}

/// 3 x 3 matrix
pub struct Mat3
{
  // row 0
  x0: f32,
  y0: f32,
  z0: f32,

  // row 1
  x1: f32,
  y1: f32,
  z1: f32,

  // row 2
  x2: f32,
  y2: f32,
  z2: f32,
}

/// matrix functions
impl Mat3
{
  /// mat3 constructor
  pub fn new(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32,) -> Mat3
  {
    Mat3 {x0, y0, z0, x1, y1, z1, x2, y2, z2}
  }

  /// copies a into self
  pub fn copy(&mut self, a: Mat3) -> &mut Self
  {
    self.x0 = a.x0;
    self.y0 = a.y0;
    self.z0 = a.z0;

    self.x1 = a.x1;
    self.y1 = a.y1;
    self.z1 = a.z1;

    self.x2 = a.x2;
    self.y2 = a.y2;
    self.z2 = a.z2;

    self
  }

  /// multiplies self * a
  pub fn mult(&mut self, a: Mat3) -> &mut Self
  {
    let r: Mat3 = Mat3::new 
    (
      self.x0 * a.x0 + self.y0 * a.x1 + self.z0 * a.x2,
      self.x0 * a.y0 + self.y0 * a.y1 + self.z0 * a.y2,
      self.x0 * a.z0 + self.y0 * a.z1 + self.z0 * a.z2,

      self.x1 * a.x0 + self.y1 * a.x1 + self.z1 * a.x2,
      self.x1 * a.y0 + self.y1 * a.y1 + self.z1 * a.y2,
      self.x1 * a.z0 + self.y1 * a.z1 + self.z1 * a.z2,

      self.x2 * a.x0 + self.y2 * a.x1 + self.z2 * a.x2,
      self.x2 * a.y0 + self.y2 * a.y1 + self.z2 * a.y2,
      self.x2 * a.z0 + self.y2 * a.z1 + self.z2 * a.z2,
    );
    
    self.copy(r); // copying r into self

    return self
  }

  /// creates rotation matrix and multiplies it by self
  pub fn rotation(&mut self, axis: Axis, ang: f32) -> &mut Self
  {
    // create rotation matrix
    let rot_mat: Mat3;
    match axis
    {
      Axis::X => rot_mat = Mat3::new
      (
        1., 0., 0., 
        0., ang.cos(), -ang.sin(),
        0., ang.sin(), ang.cos(),
      ),

      Axis::Y => rot_mat = Mat3::new
      (
        ang.cos(), 0., ang.sin(),
        0., 1., 0., 
        -ang.sin(), 0., ang.cos(),
      ),

      Axis::Z => rot_mat = Mat3::new 
      (
        ang.cos(), -ang.sin(), 0.,
        ang.sin(), ang.cos(), 0.,
        0., 0., 1.,
      ),
    }

    // multiply rotation matrix by self and return
    self.mult(rot_mat)
  }

  // can i overload operators here??
}

/// 3 x 1 vector
pub struct Vec3
{
  x: f32,
  y: f32,
  z: f32,
}

/// vector functions
impl Vec3
{
  /// vec3 constructor
  pub fn new(x: f32, y: f32, z: f32) -> Vec3
  {
    Vec3 {x, y, z}
  }

  /// adds transforms to vector and returns 
  pub fn translate(&mut self, xt: f32, yt: f32, zt: f32) -> &mut Self
  {
    self.x += xt;
    self.y += yt;
    self.z += zt;

    self
  }
}

/// overloading matrix * vector multiplication
/// results in a 3 x 1 vector
impl Mul<Vec3> for Mat3
{
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Vec3
  {
    let rv: Vec3 = Vec3::new
    (
      self.x0 * rhs.x + self.y0 * rhs.y + self.z0 * rhs.z,
      self.x1 * rhs.x + self.y1 * rhs.y + self.z1 * rhs.z,
      self.x2 * rhs.x + self.y2 * rhs.y + self.z2 * rhs.z
    );

    rv
  }
}