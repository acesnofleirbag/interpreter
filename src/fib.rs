use num_bigint::BigInt;

type Matrix2x2 = [[BigInt; 2]; 2];

#[allow(non_snake_case)]
fn __matrix__x(a: &Matrix2x2, b: &Matrix2x2) -> Matrix2x2 {
    let x00 = &a[0][0] * &b[0][0] + &a[0][1] * &b[1][0];
    let x01 = &a[0][0] * &b[0][1] + &a[0][1] * &b[1][1];
    let x10 = &a[1][0] * &b[0][0] + &a[1][1] * &b[1][0];
    let x11 = &a[1][0] * &b[0][1] + &a[1][1] * &b[1][1];

    [[x00, x01], [x10, x11]]
}

fn __pow(matrix: &Matrix2x2, nth: BigInt) -> Matrix2x2 {
    if &nth == &BigInt::from(1) {
        matrix.clone()
    } else if &nth % 2 == BigInt::from(0) {
        let half_pow = __pow(matrix, nth / 2);

        __matrix__x(&half_pow, &half_pow)
    } else {
        __matrix__x(&matrix, &__pow(&matrix, nth - 1))
    }
}

pub fn __fib_matrix(nth: BigInt) -> BigInt {
    let init = [[BigInt::from(1), BigInt::from(1)], [BigInt::from(1), BigInt::from(0)]];
    let res = __pow(&init, nth);

    res[1][0].clone()
}

pub fn __fib_iter(nth: BigInt) -> BigInt {
    let mut a: BigInt = BigInt::from(0);
    let mut b: BigInt = BigInt::from(1);

    let mut i: BigInt = BigInt::from(0);
    while i < nth {
        [b, a] = [&a + &b, b];

        i += 1;
    }

    a
}
