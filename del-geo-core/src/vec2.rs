//! methods for 2D vector
/// trait for 2D vector
pub trait Vec2<Real>
where
    Self: Sized,
{
    fn sub(&self, other: &Self) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn transform_homogeneous(&self, v: &[Real; 9]) -> Option<[Real; 2]>;
}

impl<Real> Vec2<Real> for [Real; 2]
where
    Real: num_traits::Float,
{
    fn sub(&self, other: &Self) -> Self {
        sub(self, other)
    }
    fn add(&self, other: &Self) -> Self {
        add(self, other)
    }
    fn transform_homogeneous(&self, v: &[Real; 9]) -> Option<Self> {
        crate::mat3_col_major::transform_homogeneous(v, self)
    }
}

pub fn length<Real>(p: &[Real; 2]) -> Real
where
    Real: num_traits::Float,
{
    (p[0] * p[0] + p[1] * p[1]).sqrt()
}

pub fn squared_length<Real>(p: &[Real; 2]) -> Real
where
    Real: num_traits::Float,
{
    p[0] * p[0] + p[1] * p[1]
}

pub fn sub<T>(a: &[T; 2], b: &[T; 2]) -> [T; 2]
where
    T: std::ops::Sub<Output = T> + Copy,
{
    [a[0] - b[0], a[1] - b[1]]
}

pub fn add<T>(a: &[T; 2], b: &[T; 2]) -> [T; 2]
where
    T: std::ops::Add<Output = T> + Copy,
{
    [a[0] + b[0], a[1] + b[1]]
}

pub fn scale<T>(a: &[T; 2], s: T) -> [T; 2]
where
    T: num_traits::Float,
{
    [a[0] * s, a[1] * s]
}

pub fn dot<T>(a: &[T; 2], b: &[T; 2]) -> T
where
    T: num_traits::Float,
{
    a[0] * b[0] + a[1] * b[1]
}

pub fn area_quadrilateral<T>(a: &[T; 2], b: &[T; 2]) -> T
where
    T: num_traits::Float,
{
    a[0] * b[1] - a[1] * b[0]
}

pub fn angle_between_two_vecs<T>(a: &[T; 2], b: &[T; 2]) -> T
where
    T: num_traits::Float,
{
    let dot = dot(a, b);
    let area = area_quadrilateral(a, b);
    area.atan2(dot)
}

#[test]
fn test_angle_between_two_vecs() {
    let a = [3f64.sqrt(), 1.0];
    let b = [-1.0, 1.0];
    let theta0 = angle_between_two_vecs(&a, &b);
    let theta1 = 7f64 / 12f64 * std::f64::consts::PI;
    assert!((theta0 - theta1).abs() < 1.0e-10);
}

pub fn wdw_angle_between_two_vecs<T>(u: &[T; 2], v: &[T; 2]) -> (T, [[T; 2]; 2])
where
    T: num_traits::Float,
{
    let a = dot(u, v);
    let b = area_quadrilateral(u, v);
    let w = b.atan2(a);
    let tmp0 = T::one() / (a * a + b * b);
    let dw_da = -b * tmp0;
    let dw_db = a * tmp0;
    let dw_du = [dw_da * v[0] + dw_db * v[1], dw_da * v[1] - dw_db * v[0]];
    let dw_dv = [dw_da * u[0] - dw_db * u[1], dw_da * u[1] + dw_db * u[0]];
    (w, [dw_du, dw_dv])
}

#[test]
fn test_wdw_angle_between_two_vecs() {
    let a0 = [[3f64.sqrt(), 1.0], [-1.0, 1.0]];
    let (t0, dt0) = wdw_angle_between_two_vecs(&a0[0], &a0[1]);
    let eps = 1.0e-5;
    for (ino, idim) in itertools::iproduct!(0..2, 0..2) {
        let a1 = {
            let mut a1 = a0;
            a1[ino][idim] += eps;
            a1
        };
        let (t1, _dt1) = wdw_angle_between_two_vecs(&a1[0], &a1[1]);
        let v0 = (t1 - t0) / eps;
        let v1 = dt0[ino][idim];
        assert!((v0 - v1).abs() < 1.0e-5);
    }
}

pub fn from_homogeneous<Real>(v: &[Real; 3]) -> Option<[Real; 2]>
where
    Real: num_traits::Float,
{
    if v[2].is_zero() {
        return None;
    }
    Some([v[0] / v[2], v[0] / v[2]])
}

pub fn rotate(p: &[f32; 2], theta: f32) -> [f32; 2] {
    let c = theta.cos();
    let s = theta.sin();
    [c * p[0] - s * p[1], s * p[0] + c * p[1]]
}

pub fn normalize(p: &[f32; 2]) -> [f32; 2] {
    let invl = 1.0 / (p[0] * p[0] + p[1] * p[1]).sqrt();
    [p[0] * invl, p[1] * invl]
}

pub fn orthogonalize(u: &[f32; 2], v: &[f32; 2]) -> [f32; 2] {
    let t = dot(u, v) / dot(u, u);
    [v[0] - t * u[0], v[1] - t * u[1]]
}

pub fn axpy<Real>(alpha: Real, x: &[Real; 2], y: &[Real; 2]) -> [Real; 2]
where
    Real: num_traits::Float,
{
    [alpha * x[0] + y[0], alpha * x[1] + y[1]]
}

// -------------------------------
// below: about the Vec2 class
pub struct XY<'a, Real> {
    pub p: &'a [Real; 2],
}

#[allow(clippy::needless_lifetimes)]
impl<'a, Real> XY<'a, Real>
where
    Real: num_traits::Float,
{
    pub fn aabb(&self) -> [Real; 4] {
        [self.p[0], self.p[1], self.p[0], self.p[1]]
    }
}
