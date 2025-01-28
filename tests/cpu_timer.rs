//a Imports
use std::collections::HashMap;

use cpu_timer::{AccTimer, AccTrace, DeltaTimer, TArch, TDesc, Timer, Trace};

//a Work functions
//fp do_work
fn do_work<const S: bool>() -> ()
where
    TDesc<S>: TArch,
{
    let mut t = DeltaTimer::<S>::default();
    for _ in 0..100 {
        t.start();
        t.stop();
    }
}

//fp trace_work
fn trace_work<const S: bool>(t: &mut Trace<S, u32, 16>) -> ()
where
    TDesc<S>: TArch,
{
    t.start();
    for i in 0..16 {
        let mut ti = DeltaTimer::<S>::default();
        for _ in 0..100 * (i + 1) {
            ti.start();
            ti.stop();
        }
        t.next();
    }
}

//fp acc_trace_work
fn acc_trace_work<const S: bool>(t: &mut AccTrace<S, u32, 16>) -> ()
where
    TDesc<S>: TArch,
{
    for _acc in 0..10 {
        t.start();
        for i in 0..16 {
            let mut ti = DeltaTimer::<S>::default();
            for _ in 0..100 * (i + 1) {
                ti.start();
                ti.stop();
            }
            t.next();
        }
        t.acc();
    }
}

//a Useful functions
//fp abs_diff
fn abs_diff(a: u64, b: u64) -> u64 {
    ((a as i64) - (b as i64)).abs() as u64
}

//fp check_data
fn check_data(data: &[u32]) -> Result<(), String> {
    let n = data.len() as u64;
    let mut acc = 0;
    for d in data {
        acc += (*d) as u64;
    }
    let mut zeros = 0;
    let mut outliers = 0;
    for d in data {
        let v = ((*d) as u64) * n;
        if v == 0 {
            zeros += 1;
        }
        if v < acc * 80 / 100 {
            outliers += 1;
        }
        if v > acc * 120 / 100 {
            outliers += 1;
        }
    }
    if zeros > 0 {
        Err(format!("Too many zeros {zeros}"))
    } else if outliers > 1 {
        Err(format!("More than one outliers {outliers}"))
    } else {
        Ok(())
    }
}

//a Generic tests
//fp generic_test_timer
fn generic_test_timer<const S: bool>()
where
    TDesc<S>: TArch,
{
    let mut t0 = Timer::<S>::default();
    let mut t1 = Timer::<S>::default();
    t0.start();
    t1.start();
    do_work::<S>();
    let t0_value = t0.elapsed();
    let t1_value = t1.elapsed();

    let diff = abs_diff(t0_value, t1_value);
    dbg!(&t0, &t1);
    assert!(t0_value != 0, "Value should not be 0 after work");
    assert!(
        diff < t0_value * 20 / 100,
        "Expecting the difference to be more less than 20% of the value; this might fail very infrequently"
    );
}

//fp generic_test_delta_timer
fn generic_test_delta_timer<const S: bool>()
where
    TDesc<S>: TArch,
{
    let mut t0 = DeltaTimer::<S>::default();
    let mut t1 = DeltaTimer::<S>::default();
    t0.start();
    t1.start();
    do_work::<S>();
    t0.stop();
    t1.stop();

    let diff = abs_diff(t0.value(), t1.value());
    dbg!(&t0, &t1);
    assert!(t0.value() != 0, "Value should not be 0 after work");
    assert!(
        diff < t0.value() * 20 / 100,
        "Expecting the difference to be more less than 20% of the value; this might fail very infrequently"
    );

    t0.clear();
    assert_eq!(t0.value(), 0, "Value should be 0 at clear");
}

//fp generic_test_trace
fn generic_test_trace<const S: bool>() -> Result<(), String>
where
    TDesc<S>: TArch,
{
    let mut result = Ok(());
    for _retries in 0..10 {
        let mut t0 = Trace::<S, u32, 16>::default();
        trace_work(&mut t0);
        let mut samples: Vec<u32> = vec![];
        for (i, t) in t0.trace().iter().enumerate() {
            samples.push(t / (i + 1) as u32);
        }
        result = check_data(&samples);
        if result.is_ok() {
            break;
        }
    }
    result
}

//fp generic_test_acc_trace
fn generic_test_acc_trace<const S: bool>() -> Result<(), String>
where
    TDesc<S>: TArch,
{
    let mut result = Ok(());
    for _retries in 0..10 {
        let mut t0 = AccTrace::<S, u32, 16>::default();
        acc_trace_work(&mut t0);
        let mut samples: Vec<u32> = vec![];
        for (i, t) in t0.acc_trace().iter().enumerate() {
            samples.push(t / (i + 1) as u32);
        }
        result = check_data(&samples);
        if result.is_ok() {
            break;
        }
    }
    if result.is_err() {
        return result;
    }
    for _retries in 0..10 {
        let mut t0 = AccTrace::<S, u32, 16>::default();
        acc_trace_work(&mut t0);
        let mut samples: Vec<u32> = vec![];
        for (i, t) in t0.last_trace().iter().enumerate() {
            samples.push(t / (i + 1) as u32);
        }
        result = check_data(&samples);
        if result.is_ok() {
            break;
        }
    }
    result
}

//fp generic_test_acc_timer
fn generic_test_acc_timer<const S: bool>()
where
    TDesc<S>: TArch,
{
    const N: usize = 10_000;
    let mut t0 = AccTimer::<S>::default();
    let mut passed = false;

    let mut acc_10x = 0;
    for _retries in 0..10 {
        let t_acc_x10 = {
            for _ in 0..N * 10 {
                t0.start();
                do_work::<S>();
                t0.stop();
            }
            t0.acc_value()
        };
        t0.clear();
        let t_acc_10x = {
            let mut acc = 0;
            for _ in 0..10 {
                for _ in 0..N {
                    t0.start();
                    do_work::<S>();
                    t0.stop();
                }
                acc += t0.acc_value();
                t0.clear();
            }
            acc
        };

        assert!(t_acc_10x != 0, "10*N iterations cannot be zero");
        let diff = abs_diff(t_acc_x10, t_acc_10x);
        dbg!(diff, t_acc_x10, t_acc_10x);
        if diff < t_acc_10x * 1 / 10 {
            passed = true;
            acc_10x = t_acc_10x;
            break;
        }
    }
    assert!(
        passed,
        "Expected diff to be within 10% in at least one attempt"
    );

    let mut zeros = 0;
    let mut outliers = 0;
    let mut total = 0;
    t0.clear();
    for _ in 0..N {
        t0.start();
        do_work::<S>();
        t0.stop();
        let v = t0.last_delta() as usize;
        total += v;
        if v == 0 {
            zeros += 1;
        }
        if v * 10 * N < acc_10x as usize * 80 / 100 {
            outliers += 1;
        }
        if v * 10 * N > acc_10x as usize * 2 {
            outliers += 1;
        }
    }
    assert_eq!(
        total,
        t0.acc_value() as usize,
        "Accumulator should equal the sum of values"
    );

    dbg!(acc_10x, zeros, outliers, (acc_10x as usize) / 10 / N);

    assert!(zeros != N, "Cannot all be zero");
    assert!(outliers < N * 1 / 10, "Fewer than 10% should be outliers");

    // assert!(false);
}

//fp generic_test_timer_values
fn generic_test_timer_values<const S: bool>()
where
    TDesc<S>: TArch,
{
    // Simply record the time taken (apparenlty) by Timer::delta()
    //
    // Outputs the distribuion of delays

    let mut record = vec![0u64; 100_000];
    let mut t0 = DeltaTimer::<S>::default();
    t0.start();
    for i in 0..record.len() {
        record[i] = t0.delta();
    }

    let mut deltas: HashMap<u64, usize> = HashMap::new();

    for i in 1..record.len() {
        let d = record[i] - record[i - 1];
        *deltas.entry(d).or_default() += 1;
    }

    let mut d: Vec<u64> = deltas.keys().copied().collect();
    d.sort();
    eprintln!("Complete delta distribution");
    for d in &d {
        eprintln!("{d} {}", deltas[d]);
    }

    eprintln!("Percentile distribution");
    let mut p = 0;
    let mut acc = 0;
    let mut sum_to_95 = 0;
    let mut num_to_95 = 0;
    let n = record.len();
    for d in &d {
        acc += deltas[d];
        let acc_p = (acc * 100) / n;
        if acc_p > p {
            p = acc_p;
            eprintln!("{acc_p}, {d}");
        }
        if acc_p < 95 {
            sum_to_95 += (*d as usize) * deltas[d];
            num_to_95 += deltas[d];
        }
    }
    eprintln!("100, {}", d.last().unwrap());
    eprintln!("");
    eprintln!("average of up to 95 {}", sum_to_95 / num_to_95);
}

//a Tests
//fp test_timer
#[test]
fn test_timer() {
    generic_test_timer::<true>();
    generic_test_timer::<false>();
}

//fp test_delta_timer
#[test]
fn test_delta_timer() {
    generic_test_delta_timer::<true>();
    generic_test_delta_timer::<false>();
}

//fp test_trace
#[test]
fn test_trace() -> Result<(), String> {
    generic_test_trace::<true>()?;
    generic_test_trace::<false>()?;
    Ok(())
}

//fp test_acc_trace
#[test]
fn test_acc_trace() -> Result<(), String> {
    generic_test_acc_trace::<true>()?;
    generic_test_acc_trace::<false>()?;
    Ok(())
}

//fp test_acc_timer
#[test]
fn test_acc_timer() {
    generic_test_acc_timer::<true>();
    generic_test_acc_timer::<false>();
}

//fp test_timer_values
#[test]
fn test_timer_values() {
    generic_test_timer_values::<true>();
    generic_test_timer_values::<false>();
}
