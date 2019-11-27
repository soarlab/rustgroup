extern crate rayon;
use rayon::prelude::*;

const N: usize = 512;

// Use rayon to make a parallel matrix vector multiplication.
fn mat_vec_par(a: &[[f32; N]], b: &[f32], c: &mut [f32]) {
    let chunk_size=4;
    // Use par_chunks on a and par_chunks_mut on c
    // then use zip to iterate over the chunks
    // pairwise. Use the for_each method to
    // perform the mat-vec between the a chunk and b
    // and store the result in the c chunk.
    // https://docs.rs/rayon/1.2.1/rayon/slice/trait.ParallelSlice.html#method.par_chunks
    // https://docs.rs/rayon/1.2.1/rayon/slice/trait.ParallelSliceMut.html#method.par_chunks_mut
    // https://docs.rs/rayon/1.2.1/rayon/iter/trait.IndexedParallelIterator.html#method.zip
    // https://docs.rs/rayon/1.2.1/rayon/iter/trait.ParallelIterator.html#method.for_each
}

fn mat_vec(a: &[[f32; N]], b: &[f32], c: &mut [f32]) {
    let n = a.len();
    for i in 0..n {
        for j in 0..n {
            c[i] += a[i][j]*b[j];
        }
    }
}

fn main() {
    let mut a = [[0.0; N]; N];
    let mut b = [0.0; N];
    let mut c = [0.0; N];
    for i in 0..N {
        b[i] = (i*3) as f32;
        for j in 0..N {
            a[i][j] = (i*N + j) as f32;
        }
    }
    
    let pa = a.clone();
    let pb = b.clone();
    let mut pc = c.clone();

    mat_vec(&a, &b, &mut c);
    
    mat_vec_par(&pa, &pb, &mut pc);

    for i in c.iter().zip(pc.iter()) {
        assert_eq!(i.0, i.1);
    }
}
