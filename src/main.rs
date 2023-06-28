use std::time::Instant;
fn main() {
    const N: usize = 1500;
    let mut a = [[0u8; N]; N];
    let mut b = [[0u8; N]; N];
    for i in 0..N {
        for j in 0..N {
            a[i][j] = rand::random::<u8>();
            b[i][j] = rand::random::<u8>();
        }
    }
    let mut c = [[0u8; N]; N];

    let t0 = Instant::now();
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                c[i][j] = c[i][j].wrapping_add(a[i][k].wrapping_mul(b[k][j]))
            }
        }
    }
    let t = t0.elapsed();
    println!("{}", t.as_nanos());

    let t0 = Instant::now();
    for i in 0..N {
        for k in 0..N {
            for j in 0..N {
                c[i][j] = c[i][j].wrapping_add(a[i][k].wrapping_mul(b[k][j]))
            }
        }
    }
    let t = t0.elapsed();
    println!("{}", t.as_nanos());

    let t0 = Instant::now();
    for j in 0..N {
        for i in 0..N {
            for k in 0..N {
                c[i][j] = c[i][j].wrapping_add(a[i][k].wrapping_mul(b[k][j]))
            }
        }
    }
    let t = t0.elapsed();
    println!("{}", t.as_nanos());

    let t0 = Instant::now();
    for j in 0..N {
        for k in 0..N {
            for i in 0..N {
                c[i][j] = c[i][j].wrapping_add(a[i][k].wrapping_mul(b[k][j]))
            }
        }
    }
    let t = t0.elapsed();
    println!("{}", t.as_nanos());

    let t0 = Instant::now();
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                c[i][j] = c[i][j].wrapping_add(a[i][k].wrapping_mul(b[k][j]))
            }
        }
    }
    let t = t0.elapsed();
    println!("{}", t.as_nanos());

    let t0 = Instant::now();
    for k in 0..N {
        for j in 0..N {
            for i in 0..N {
                c[i][j] = c[i][j].wrapping_add(a[i][k].wrapping_mul(b[k][j]))
            }
        }
    }
    let t = t0.elapsed();
    println!("{}", t.as_nanos());
}
