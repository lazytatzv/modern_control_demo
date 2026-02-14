use std::io;
use anyhow::Result;
use std::any::Any;
use nalgebra::{DMatrix, DVector, Dyn, Matrix};

// 行列を初期化する
fn gen_matrices() -> Result<(usize, DMatrix<f64>, DMatrix<f64>, DMatrix<f64>)> {
    let mut buffer = String::new();

    println!("The dimension of the system(n): ");

    io::stdin().read_line(&mut buffer)?;

    let dim: usize = buffer.trim().parse()?;

    //println!("{}", dim.type_id());

    let mut a = DMatrix::<f64>::zeros(dim, dim);
    //vector
    let mut b = DMatrix::<f64>::zeros(dim, 1);
    let mut c = DMatrix::<f64>::zeros(1, dim);

    //println!("{:?}", A);

    for i in 0..dim {
        for j in 0..dim {

            let elem: f64 = loop {
                println!("Elem of A[{}, {}]", i, j);
                let mut buf = String::new();
                io::stdin().read_line(&mut buf)?;
                
                match buf.trim().parse::<f64>() {
                    Ok(elem) => {
                        break elem;
                    },
                    Err(_) => {
                        println!("Invalid input.");
                        continue;
                    }
                };

            };
            a[(i, j)] = elem;

        }
    }
    println!("Your system matrix is {}", a);

    for i in 0..dim {
        let elem: f64 = loop {
            println!("Elem of B[{}, 0]", i);
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;

            match buf.trim().parse::<f64>() {
                Ok(elem) => {
                    break elem;
                },
                Err(_) => {
                    println!("Invalid input.");
                    continue;
                }
            };
        };

        b[(i, 0)] = elem;
    }
    println!("Your vector B is {}", b);

    for i in 0..dim {

        let elem: f64 = loop {
            println!("Elem of C[0, {}]", i);
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;

            match buf.trim().parse::<f64>() {
                Ok(elem) => {
                    break elem;
                },
                Err(_) => {
                    println!("Invalid input.");
                    continue;
                }
            };
        };

        c[(0, i)] = elem;
    }
    println!("Your vector C is {}", c);


    Ok((dim, a, b, c))
}

// 行列のn乗
// 計算量O(k)
// pow()
fn power_matrix(m: DMatrix<f64>, pow: usize) -> Result<DMatrix<f64>>{
   let mut result = m.clone(); 

   for _ in 0..(pow-1) {
        result *= m.clone();
   }

   Ok(result)

}

// 横連結
fn hstack(x: &DMatrix<f64>, y: &DMatrix<f64>) -> Result<DMatrix<f64>> {
    let (rows_x, cols_x) = x.shape();
    let (rows_y, cols_y) = y.shape();

    if rows_x != rows_y {
        anyhow::bail!("The rows are different!");
    }

    let mut m = DMatrix::<f64>::zeros(rows_x, cols_x + cols_y);

    m.slice_mut((0, 0), (rows_x, cols_x)).copy_from(&x);
    m.slice_mut((0, cols_x), (rows_x, cols_y)).copy_from(&y);

    Ok(m)
}

// 縦連結
fn vstack(x: &DMatrix<f64>, y: &DMatrix<f64>) -> Result<DMatrix<f64>> {
    let (rows_x, cols_x) = x.shape();
    let (rows_y, cols_y) = y.shape();

    if cols_x != cols_y {
        anyhow::bail!("The columns are different!");
    }

    let mut m = DMatrix::<f64>::zeros(rows_x + rows_y, cols_x);

    m.slice_mut((0, 0), (rows_x, cols_x)).copy_from(&x);
    m.slice_mut((rows_x, 0), (rows_y, cols_x)).copy_from(&y);

    Ok(m)
}

// rank計算
fn calc_rank(mut x: DMatrix<f64>) -> usize {
    let tol = 1e-10;
    let mut rank = 0;

    let nrows = x.nrows();
    let ncols = x.ncols();
    let mut row = 0;

    for col in 0..ncols {

        let mut pivot_row = None;

        for r in row..nrows {
            if x[(r, col)].abs() > tol {
                pivot_row = Some(r);
                break;
            }
        }

        if let Some(pivot) = pivot_row {

            if pivot != row {
                /*
                let tmp = x.row(row).clone_owned();
                x.row_mut(row).copy_from(&x.row(pivot));
                x.row_mut(pivot).copy_from(&tmp);
                */
                x.swap_rows(row, pivot);
            }

            
            let pivot_val = x[(row, col)];
            x.row_mut(row).scale_mut(1.0 / pivot_val);


            for r in (row + 1)..nrows {
                let factor = x[(r, col)];
                let scaled_row = x.row(row) * factor;
                let mut target_row = x.row_mut(r);
                target_row -= &scaled_row;
            }

            row += 1;
            rank += 1;
        }
    }
    rank

}

// 可制御判定
fn check_controllability(dim: usize, a: &DMatrix<f64>, b: &DMatrix<f64>) -> Result<String> {
    let mut m = b.clone();

    for i in 1..dim {
        let mut frag = a.pow((i as usize).try_into().unwrap()) * b;

        m = hstack(&m, &frag)?; 
    }

    if calc_rank(m.clone()) != dim {
        anyhow::bail!("Not controllable!");
    }

    println!("matrix:{:?}, rank: {}", m.clone(), calc_rank(m));

    Ok("Ok, Controllable!".into())

}

// 可観測判定
fn check_observability(dim: usize, a: &DMatrix<f64>, c: &DMatrix<f64>) -> Result<String> {
    let mut m = c.clone();

    for i in 1..dim {
        let mut frag = c * a.pow((i as usize).try_into().unwrap());

        m = vstack(&m, &frag)?;
    }

    if calc_rank(m) != dim {
        anyhow::bail!("Not observable!");
    }

    Ok("Ok, Observable".into())

}


fn main() -> Result<()> {

    let (dim, mut a, mut b, mut c) = gen_matrices()?;

    //println!("{}", power_matrix(a.clone(), 2)?);
    //println!("{}", a.clone().pow(2));
    //println!("{}", hstack(&a, &b)?);
    //println!("{}", vstack(&a, &b)?);
 
    let eig = a.clone().symmetric_eigen();

    for val in eig.eigenvalues.iter() {
        if *val >= 0.0 {
            eprintln!("The system is not asymptotically stable!!");
            println!("{}", *val);
            return Ok(());
        }
    } 
    println!("The system is stable");

    println!("Controllability: {}", check_controllability(dim, &a, &b)?);
    println!("Observability: {}", check_observability(dim, &a, &c)?);

    Ok(())

}
