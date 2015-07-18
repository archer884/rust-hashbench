extern crate farmhash;
extern crate time;
extern crate fnv;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use time::precise_time_s;
use std::hash::{Hash, SipHasher, Hasher};


fn main() {
    let path = Path::new("/usr/share/dict/web2a");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };


    let mut web2 = String::new();
    match file.read_to_string(&mut web2) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {},
    }

    {
        let split: Vec<&str> = web2.split("\n").collect();
        let split_len = split.len();
        let mut v: Vec<u64> = vec![];
        let mut needed_time: f64 = 0.0;

        for s in split {
            let starttime = precise_time_s();
            let res = farmhash::hash64(&s.as_bytes());
            needed_time += precise_time_s() - starttime;
            v.push(res);
        }
        v.sort();
        v.dedup();
        let dedup_percent = split_len - v.len();
        println!("farmhash required {} s with {}/{} collisions", needed_time, dedup_percent, split_len);
    }


    {
        let split: Vec<&str> = web2.split("\n").collect();
        let split_len = split.len();
        let mut v: Vec<u64> = vec![];
        let mut needed_time: f64 = 0.0;

        for s in split {
            let starttime = precise_time_s();
            let mut hasher = fnv::FnvHasher::default();
            s.hash(&mut hasher);
            let res = hasher.finish();
            needed_time += precise_time_s() - starttime;
            v.push(res);
        }

        v.sort();
        v.dedup();
        let dedup_percent = split_len - v.len();
        println!("fnv required {} s with {}/{} collisions", needed_time, dedup_percent, split_len);
    }

    {
        let split: Vec<&str> = web2.split("\n").collect();
        let split_len = split.len();
        let mut v: Vec<u64> = vec![];
        let mut needed_time: f64 = 0.0;

        for s in split {
            let starttime = precise_time_s();
            let mut hasher = SipHasher::default();
            s.hash(&mut hasher);
            let res = hasher.finish();
            needed_time += precise_time_s() - starttime;
            v.push(res);
        }

        v.sort();
        v.dedup();
        let dedup_percent = split_len - v.len();
        println!("siphash required {} s with {}/{} collisions", needed_time, dedup_percent, split_len);
    }
}