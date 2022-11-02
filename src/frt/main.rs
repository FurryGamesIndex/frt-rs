#![allow(dead_code, unused_variables)]

use libfrt::profile::Profile;

fn main() {
    let profile = Profile::from_config("1.toml").unwrap();

    println!("{:?}", profile);
}