use rayon::prelude::*;
use std::time::Instant;
use rand::prelude::*;


pub fn main(){
    let start = Instant::now();

    //! rayon parellelism is usully used in large amount of data, bc if it's used in a rnage of 10 the stard way will perform better
    let sum:u64 = (0..10_000_000)
        .into_par_iter()
        .map(|i| {
            i * rand::random_range(0..1000)
        })
        .sum();

    dbg!(start.elapsed());
    dbg!(sum);

}