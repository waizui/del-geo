pub fn mult_vec3<Real>(a: &[Real; 6], b: &[Real; 3]) -> [Real; 2]
where
    Real: num_traits::Float,
{
    [
        a[0] * b[0] + a[2] * b[1] + a[4] * b[2],
        a[1] * b[0] + a[3] * b[1] + a[5] * b[2],
    ]
}

pub fn mult_mat3_col_major<Real>(a: &[Real; 6], b: &[Real; 9]) -> [Real; 6]
where
    Real: num_traits::Float,
{
    [
        a[0] * b[0] + a[2] * b[1] + a[4] * b[2],
        a[1] * b[0] + a[3] * b[1] + a[5] * b[2],
        a[0] * b[3] + a[2] * b[4] + a[4] * b[5],
        a[1] * b[3] + a[3] * b[4] + a[5] * b[5],
        a[0] * b[6] + a[2] * b[7] + a[4] * b[8],
        a[1] * b[6] + a[3] * b[7] + a[5] * b[8],
    ]
}

pub fn transform_ndc2pix(img_shape: (usize, usize)) -> [f32; 6] {
    [
        0.5 * (img_shape.0 as f32),
        0.,
        0.,
        -0.5 * (img_shape.1 as f32),
        0.5 * (img_shape.0 as f32),
        0.5 * (img_shape.1 as f32),
    ]
}
