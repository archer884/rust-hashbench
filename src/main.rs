extern crate farmhash;
extern crate fnv;
extern crate metrohash;
extern crate time;

use std::fs::File;
use std::hash::{Hash, SipHasher, Hasher};
use std::io::prelude::*;
use time::precise_time_s;

const DICT_PATH: &'static str = "/usr/share/dict/web2a";

fn main() {
    let mut file = File::open(DICT_PATH)
        .expect(&format!("couldn't open {}", DICT_PATH));

    let mut web2 = String::new();
    file.read_to_string(&mut web2)
        .expect(&format!("couldn't read {}", DICT_PATH));

    let data: Vec<&str> = web2.split('\n').collect();

    run_test("farmhash", &data, farmhash::FarmHasher::default);
    run_test("fnv", &data, fnv::FnvHasher::default);
    run_test("siphash", &data, SipHasher::new);
    run_test("metrohash", &data, metrohash::MetroHash64::default);
}

fn run_test<F, T>(name: &str, data: &[&str], factory: F) 
    where F: Fn() -> T,
          T: Hasher,
{
    let mut hashes = Vec::new();
    let mut time = 0.0f64;

    for item in data {
        let starttime = precise_time_s();
        let mut hasher = factory();
        item.hash(&mut hasher);
        let res = hasher.finish();
        time += precise_time_s() - starttime;
        hashes.push(res);
    }

    hashes.sort();
    hashes.dedup();
    let dedup_percent = data.len() - hashes.len();
    println!("{} required {} s with {}/{} collisions", name, time, dedup_percent, data.len());
}
