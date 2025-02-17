use crate::vec3::Vec3;
/// trait for 3x3 matrix in row-major order
pub trait Mat3RowMajor<Real: num_traits::Float> {
    fn from_identity() -> Self;
    fn determinant(&self) -> Real;
    fn squared_norm(&self) -> Real;
    fn transpose(&self) -> Self;
    fn mult_mat_row_major(&self, b: &Self) -> Self;
    fn sub(&self, b: &Self) -> Self;
}
impl<Real> Mat3RowMajor<Real> for [Real; 9]
where
    Real: num_traits::Float + std::ops::AddAssign + std::iter::Sum,
{
    fn from_identity() -> Self {
        from_identity()
    }
    fn determinant(&self) -> Real {
        determinant(self)
    }
    fn squared_norm(&self) -> Real {
        squared_norm(self)
    }
    fn transpose(&self) -> Self {
        transpose(self)
    }
    fn mult_mat_row_major(&self, b: &Self) -> Self {
        mult_mat_row_major(self, b)
    }
    fn sub(&self, b: &Self) -> Self {
        sub(self, b)
    }
}

pub fn from_identity<T>() -> [T; 9]
where
    T: num_traits::Float,
{
    let zero = T::zero();
    let one = T::one();
    [one, zero, zero, zero, one, zero, zero, zero, one]
}

// above: from method
// --------------------------

pub fn determinant<Real>(u: &[Real; 9]) -> Real
where
    Real: num_traits::Float,
{
    u[0] * u[4] * u[8] + u[3] * u[7] * u[2] + u[6] * u[1] * u[5]
        - u[0] * u[7] * u[5]
        - u[6] * u[4] * u[2]
        - u[3] * u[1] * u[8]
}

pub fn squared_norm<Real>(u: &[Real; 9]) -> Real
where
    Real: num_traits::Float + std::iter::Sum,
{
    u.iter().map(|&v| v * v).sum()
}

pub fn transpose<Real>(m: &[Real; 9]) -> [Real; 9]
where
    Real: num_traits::Float,
{
    [m[0], m[3], m[6], m[1], m[4], m[7], m[2], m[5], m[8]]
}

pub fn mult_mat_row_major<Real>(a: &[Real; 9], b: &[Real; 9]) -> [Real; 9]
where
    Real: num_traits::Float + std::ops::AddAssign,
{
    let mut r = [Real::zero(); 9];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                r[i * 3 + j] += a[i * 3 + k] * b[k * 3 + j]
            }
        }
    }
    r
}

pub fn sub<Real>(a: &[Real; 9], b: &[Real; 9]) -> [Real; 9]
where
    Real: num_traits::Float,
{
    std::array::from_fn(|i| a[i] - b[i])
}

fn sort_eigen(g: &mut [f64; 3], v: &mut [f64; 9]) {
    if g[1] > g[0] {
        g.swap(0, 1);
        v.swap(0, 1);
        v.swap(3, 4);
        v.swap(6, 7);
    }
    if g[2] > g[1] {
        g.swap(1, 2);
        v.swap(1, 2);
        v.swap(4, 5);
        v.swap(7, 8);
    }
    if g[1] > g[0] {
        g.swap(0, 1);
        v.swap(0, 1);
        v.swap(3, 4);
        v.swap(6, 7);
    }
}

// m = UGV^T
pub fn svd(m: &[f64; 9], nitr: usize) {
    // M^TM = VGGV^T
    let mtm = [
        m[0] * m[0] + m[3] * m[3] + m[6] * m[6],
        m[1] * m[1] + m[4] * m[4] + m[7] * m[7],
        m[2] * m[2] + m[5] * m[5] + m[8] * m[8],
        m[1] * m[2] + m[4] * m[5] + m[7] * m[8],
        m[2] * m[0] + m[5] * m[3] + m[8] * m[6],
        m[0] * m[1] + m[3] * m[4] + m[6] * m[7],
    ];
    let Some((mut v, mut lv)) = crate::mat3_sym::eigen_decomp(mtm, nitr) else {
        todo!()
    };
    sort_eigen(&mut lv, &mut v);
    lv = lv.map(|x| x.clamp(0.0, x));
    let mut g = lv.map(|x| x.sqrt());

    let mut u0 = [
        m[0] * v[0] + m[1] * v[3] + m[2] * v[6],
        m[3] * v[0] + m[4] * v[3] + m[5] * v[6],
        m[6] * v[0] + m[7] * v[3] + m[8] * v[6],
    ];
    let mut u1 = [
        m[0] * v[1] + m[1] * v[4] + m[2] * v[7],
        m[3] * v[1] + m[4] * v[4] + m[5] * v[7],
        m[6] * v[1] + m[7] * v[4] + m[8] * v[7],
    ];
    let mut u2 = [
        m[0] * v[2] + m[1] * v[5] + m[2] * v[8],
        m[3] * v[2] + m[4] * v[5] + m[5] * v[8],
        m[6] * v[2] + m[7] * v[5] + m[8] * v[8],
    ];

    let mut u = [0f64; 9];
    if v.determinant() < 0. {
        // making right hand coordinate
        v[2] *= -1.; // v[0,2] = v[0*3+2]
        v[5] *= -1.; // v[1,2] = v[1*3+2]
        v[2 * 3 + 2] *= -1.;
        g[2] *= -1.;
        u[2] *= -1.; // u[0,2] = u[0*3+2]
        u[5] *= -1.; // u[1,2] = u[1*3+2]
        u[8] *= -1.; // u[2,2] = u[2*3+2]
    }

    let sql0 = u0.squared_norm();
    if sql0 > 1.0e-20 {
        u0.normalize_in_place();
        let sql1 = u1.squared_norm();
        if sql1 < 1.0e-20 {
            u1 = u0.map(|x| 1.0 - x.abs());
        } else {
            u1.normalize_in_place();
        }
        let d01 = u0[0] * u1[0] + u0[1] * u1[1] + u0[2] * u1[2];
        u1 = std::array::from_fn(|i| u1[i] - d01 * u0[i]);
        u1.normalize_in_place();
        let s2 = u0.cross(&u1);
        let d22 = u2[0] * s2[0] + u2[1] * s2[1] + u2[2] * s2[2];
        u2 = s2;
        if d22 < 0. {
            g[2] *= -1.;
        }
    } else {
        u0 = [1., 0., 0.];
        u1 = [0., 1., 0.];
        u2 = [0., 0., 1.];
    }
    u[0] = u0[0];
    u[1] = u1[0];
    u[2] = u2[0];
    u[3] = u0[1];
    u[4] = u1[1];
    u[5] = u2[1];
    u[6] = u0[2];
    u[7] = u1[2];
    u[8] = u2[2];
}
