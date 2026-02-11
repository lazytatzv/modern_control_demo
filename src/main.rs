use std::io;
use anyhow::Result;
use std::any::Any;
use nalgebra::{DMatrix, DVector, Dyn, Matrix};

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
            println!("Elem of A[{}, {}]", i, j);
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;
            let elem: f64 = buf.trim().parse()?;
            a[(i, j)] = elem; 
        }
    }
    println!("Your system matrix is {}", a);

    for i in 0..dim {
        println!("Elem of B[{}, 0]", i);
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let elem: f64 = buf.trim().parse()?;
        b[(i, 0)] = elem;
    }
    println!("Your vector B is {}", b);

    for i in 0..dim {
        println!("Elem of C[0, {}]", i);
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let elem: f64 = buf.trim().parse()?;
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


fn check_controlability(a: DMatrix<f64>, b: DMatrix<f64>) {
    
}

fn main() -> Result<()> {

    let (dim, mut a, mut b, mut c) = gen_matrices()?;

    //println!("{}", power_matrix(a.clone(), 2)?);
    //println!("{}", a.clone().pow(2));
    println!("{}", hstack(&a, &b)?);
    println!("{}", vstack(&a, &b)?);
 
    let eig = a.symmetric_eigen();

    for val in eig.eigenvalues.iter() {
        if *val >= 0.0 {
            eprintln!("The system is not asymptotically stable!!");
            println!("{}", *val);
            return Ok(());
        }
    } 
    println!("The system is stable");

    let mut m_c = DMatrix::<f64>::zeros(dim, dim);

    for i in 0..dim {

    }

    Ok(())

}
