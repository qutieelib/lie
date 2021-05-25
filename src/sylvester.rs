use approx::assert_abs_diff_eq;
use ndarray as nd;
use num_complex::Complex64;
use std::f64::consts::PI;

///
/// Returns a su(d) basis which is generated by Sylvester's CLOCK and SHIFT matrices. Clock and
/// Shift are generalization of pauli_x and pauli_z in the su(2) case.
///
pub fn gen_sylvester(d: usize) -> Vec<nd::Array2<Complex64>> {
    let r1 = Complex64::new(1., 0.);
    let i1 = Complex64::new(0., 1.);

    // Construct SHIFT matrix
    let mut x: nd::Array2<Complex64> = nd::Array2::zeros((d, d));
    for i in 0..d - 1 {
        x[[i + 1, i]] = r1;
    }
    x[[0, d - 1]] = r1;

    // Root of unity
    let w = (2. * PI * i1 / d as f64).exp();

    // Construct CLOCK matrix
    let w_a = nd::Array1::from_shape_fn(d, |i| w.powi(i as i32));
    let z = nd::Array2::from_diag(&w_a);

    assert_abs_diff_eq!(z.dot(&x), w * x.dot(&z), epsilon = 1e-8);

    let mut basis: Vec<nd::Array2<Complex64>> = Vec::with_capacity(d.pow(2));
    basis.push(x.clone());
    basis.push(z.clone());

    let mut x_i = x.clone();
    for r in 1..d + 1 {
        let mut z_i = z.clone();
        for s in 1..d + 1 {
            // Ignore [d,1] and [1,d] since these are CLOCK and SHIFT matrices
            // Also ignore when we get an identity matrix
            if !((s == d && r == 1) || (s == 1 && r == d) || (s == d && r == d)) {
                basis.push(x_i.dot(&z_i));
            }
            z_i = z_i.dot(&z);
        }
        x_i = x_i.dot(&x);
    }

    return basis;
}
