//a Imports
use cpu_timer::{AccArray, AccVec};

#[test]
fn stuff() {
    let mut _ac = AccVec::<true, u32, u32>::with_capacity(4);
    let mut ac = AccArray::<true, f64, f64, 8>::default();
    for i in 0..10_000_000 {
        ac.start();
        for j in &["a", "", "bb", "ccc", "dddd", "eeeee", "bb", "ccc"] {
            let k = j.chars().count();
            ac.acc_n_restart(k);
        }
    }
    println!("{ac}");
    //    assert!(false);
}
