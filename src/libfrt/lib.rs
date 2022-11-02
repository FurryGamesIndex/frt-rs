#![allow(dead_code, unused_variables)]

//pub mod entries;
pub mod profile;
pub mod utils;
pub mod error;

use profile::Profile;

#[derive(Default)]
pub struct Context {
    profile: Profile
}

impl Context {
}