extern crate crossbeam;
use crossbeam::scope;
use std::time::SystemTime;

static LENGTH: usize = 100000000;
static TIMES: usize = 100;

fn main() {
    let chunk_size = LENGTH / 2;
    let time_par = {
        let mut v_parallel: Vec<f32> = (0..LENGTH).map(|val| val as f32).collect();
        let before_par = SystemTime::now();
        for _ in 0..TIMES {
            scope(|scope| {
                for mut chunk in v_parallel.chunks_mut(chunk_size) {
                    scope.spawn(move || {
                        for val in chunk.iter_mut() {
                            *val = val.sqrt();
                        }
                    });
                }
            });
        }
        let after_par = SystemTime::now();
        after_par.duration_since(before_par).unwrap()
    };
    let time_seq = {
        let mut v_seq: Vec<f32> = (0..LENGTH).map(|val| val as f32).collect();
        let before_seq = SystemTime::now();
        for _ in 0..TIMES {
            for mut chunk in v_seq.chunks_mut(chunk_size) {
                for val in chunk.iter_mut() {
                    *val = val.sqrt();
                }
            }
        }
        let after_seq = SystemTime::now();
        after_seq.duration_since(before_seq).unwrap()
    };
    println!("{:?}", time_par);
    println!("{:?}", time_seq);
    println!("Parallel speedup: {}", time_seq.as_secs() as f32 / time_par.as_secs() as f32);
}
