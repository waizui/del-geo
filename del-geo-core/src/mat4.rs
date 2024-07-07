//! methods for 4x4 matrix

pub fn transform_homogeneous<Real>(transform: &[Real; 16], x: &[Real; 3]) -> Option<[Real; 3]>
where
    Real: num_traits::Float,
{
    let y3 = transform[3] * x[0] + transform[7] * x[1] + transform[11] * x[2] + transform[15];
    if y3.is_zero() {
        return None;
    }
    //
    let y0 = transform[0] * x[0] + transform[4] * x[1] + transform[8] * x[2] + transform[12];
    let y1 = transform[1] * x[0] + transform[5] * x[1] + transform[9] * x[2] + transform[13];
    let y2 = transform[2] * x[0] + transform[6] * x[1] + transform[10] * x[2] + transform[14];
    Some([y0 / y3, y1 / y3, y2 / y3])
}

pub fn identity<Real>() -> [Real; 16]
where
    Real: num_traits::Zero + num_traits::One + Copy,
{
    let zero = Real::zero();
    let one = Real::one();
    [
        one, zero, zero, zero, zero, one, zero, zero, zero, zero, one, zero, zero, zero, zero, one,
    ]
}

pub fn diagonal<Real>(m11: Real, m22: Real, m33: Real, m44: Real) -> [Real; 16]
where
    Real: num_traits::Zero + Copy,
{
    let zero = Real::zero();
    [
        m11, zero, zero, zero, zero, m22, zero, zero, zero, zero, m33, zero, zero, zero, zero, m44,
    ]
}

pub fn try_inverse<Real>(b: &[Real; 16]) -> Option<[Real; 16]>
where
    Real: num_traits::Float + std::ops::MulAssign + std::ops::SubAssign,
{
    crate::matn::try_inverse::<Real, 4, 16>(b)
}
