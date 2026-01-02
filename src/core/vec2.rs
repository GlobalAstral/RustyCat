use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use mlua::{Table, Value};

use crate::core::core::Luable;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
  x: i32,
  y: i32
}

impl Vec2 {
  pub const ZERO: Vec2 = Self {x: 0, y: 0};
  pub const ONE: Vec2 = Self {x: 1, y: 1};

  pub fn new(x: i32, y: i32) -> Self {
    Self {x: x, y: y}
  }

  pub fn dot(self, other: Self) -> i32 {
    self.x * other.x + self.y * other.y
  }

  pub fn as_slice(&self) -> [i32; 2] {
    [self.x, self.y]
  }

  pub fn get_x(&self) -> i32 {
    self.x
  }

  pub fn get_y(&self) -> i32 {
    self.y
  }
}

impl Add for Vec2 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Vec2::new(self.x + rhs.x, self.y + rhs.y)
  }
}
impl Sub for Vec2 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    Vec2::new(self.x - rhs.x, self.y - rhs.y)
  }
}
impl Mul for Vec2 {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    Vec2::new(self.x * rhs.x, self.y * rhs.y)
  }
}
impl Mul<i32> for Vec2 {
  type Output = Self;
  fn mul(self, rhs: i32) -> Self::Output {
    Vec2::new(self.x * rhs, self.y * rhs)
  }
}
impl Mul<f32> for Vec2 {
  type Output = Self;
  fn mul(self, rhs: f32) -> Self::Output {
    Vec2::new((self.x as f32 * rhs) as i32, (self.y as f32 * rhs) as i32)
  }
}
impl Div for Vec2 {
  type Output = Self;
  fn div(self, rhs: Self) -> Self::Output {
    Vec2::new(self.x / rhs.x, self.y / rhs.y)
  }
}
impl Div<i32> for Vec2 {
  type Output = Self;
  fn div(self, rhs: i32) -> Self::Output {
    Vec2::new(self.x / rhs, self.y / rhs)
  }
}
impl Div<f32> for Vec2 {
  type Output = Self;
  fn div(self, rhs: f32) -> Self::Output {
    Vec2::new((self.x as f32 / rhs) as i32, (self.y as f32 / rhs) as i32)
  }
}
impl Neg for Vec2 {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Vec2::new(-self.x, -self.y)
  }
}


impl Add<&Vec2> for &Vec2 {
  type Output = Vec2;
  fn add(self, rhs: &Vec2) -> Self::Output {
    Vec2::new(self.x + rhs.x, self.y + rhs.y)
  }
}
impl Sub<&Vec2> for &Vec2 {
  type Output = Vec2;
  fn sub(self, rhs: &Vec2) -> Self::Output {
    Vec2::new(self.x - rhs.x, self.y - rhs.y)
  }
}
impl Mul<&Vec2> for &Vec2 {
  type Output = Vec2;
  fn mul(self, rhs: &Vec2) -> Self::Output {
    Vec2::new(self.x * rhs.x, self.y * rhs.y)
  }
}
impl Mul<i32> for &Vec2 {
  type Output = Vec2;
  fn mul(self, rhs: i32) -> Self::Output {
    Vec2::new(self.x * rhs, self.y * rhs)
  }
}
impl Mul<f32> for &Vec2 {
  type Output = Vec2;
  fn mul(self, rhs: f32) -> Self::Output {
    Vec2::new((self.x as f32 * rhs) as i32, (self.y as f32 * rhs) as i32)
  }
}
impl Div<&Vec2> for &Vec2 {
  type Output = Vec2;
  fn div(self, rhs: &Vec2) -> Self::Output {
    Vec2::new(self.x / rhs.x, self.y / rhs.y)
  }
}
impl Div<i32> for &Vec2 {
  type Output = Vec2;
  fn div(self, rhs: i32) -> Self::Output {
    Vec2::new(self.x / rhs, self.y / rhs)
  }
}
impl Div<f32> for &Vec2 {
  type Output = Vec2;
  fn div(self, rhs: f32) -> Self::Output {
    Vec2::new((self.x as f32 / rhs) as i32, (self.y as f32 / rhs) as i32)
  }
}

impl Neg for &Vec2 {
  type Output = Vec2;
  fn neg(self) -> Self::Output {
    Vec2::new(-self.x, -self.y)
  }
}

impl PartialEq for Vec2 {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}
impl Eq for Vec2 { }

impl AddAssign for Vec2 {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }    
}
impl SubAssign for Vec2 {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }   
}
impl MulAssign for Vec2 {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}
impl DivAssign for Vec2 {
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;   
  }
}
impl MulAssign<i32> for Vec2 {
  fn mul_assign(&mut self, rhs: i32) {
    *self = *self * rhs;
  }
}
impl DivAssign<i32> for Vec2 {
  fn div_assign(&mut self, rhs: i32) {
    *self = *self / rhs;   
  }
}
impl MulAssign<f32> for Vec2 {
  fn mul_assign(&mut self, rhs: f32) {
    *self = *self * rhs;
  }
}
impl DivAssign<f32> for Vec2 {
  fn div_assign(&mut self, rhs: f32) {
    *self = *self / rhs;   
  }
}

impl Luable for Vec2 {
  fn as_lua(&mut self, lua: &mlua::Lua) -> Result<mlua::Value, Box<dyn std::error::Error>> {
    let table: Table = lua.create_table()?;
    table.set("x", Value::Integer(self.get_x() as i64))?;
    table.set("y", Value::Integer(self.get_y() as i64))?;
    table.set("dot", lua.create_function(|_, (this, other): (Table, Table)| {
      let ret: i64 = this.get::<i64>("x")? * other.get::<i64>("x")? + this.get::<i64>("y")? * other.get::<i64>("y")?;
      Ok(Value::Integer(ret))
    })?)?;
    Ok(Value::Table(table))
  }

  fn from_lua(&mut self, value: Value) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(table) = value.as_table() {
      let x: i64 = table.get("x")?;
      let y: i64 = table.get("y")?;
      self.x = x as i32;
      self.y = y as i32;
      return Ok(())
    }
    Err("Invalid Lua Value".into())
  }
}
