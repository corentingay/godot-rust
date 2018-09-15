#![allow(non_snake_case)] // because of the generated bindings.

pub extern crate gdnative_core;
pub use gdnative_core as core;

use core::*;

use std::sync::{Once, ONCE_INIT};
use std::ops::*;
use libc;

include!(concat!(env!("OUT_DIR"), "/physics_types.rs"));