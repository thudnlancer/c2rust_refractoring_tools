use ndarray::{ArrayView1, ArrayView2, Array1};
use blas::{cblas_sgemv, CblasOrder, CblasTranspose};

pub fn sgemv(
    order: CblasOrder,
    trans: CblasTranspose,
    m: i32,
    n: i32,
    alpha: f32,
    a: ArrayView2<f32>,
    lda: i32,
    x: ArrayView1<f32>,
    incx: i32,
    beta: f32,
    y: &mut Array1<f32>,
    incy: i32,
) {
    cblas_sgemv(
        order,
        trans,
        m,
        n,
        alpha,
        a.as_slice().unwrap(),
        lda,
        x.as_slice().unwrap(),
        incx,
        beta,
        y.as_slice_mut().unwrap(),
        incy,
    );
}