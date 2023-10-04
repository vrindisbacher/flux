#[path = "../../lib/rvec.rs"]
mod rvec;

use rvec::RVec;

#[flux::sig(fn(&mut {RVec<i32>[@n] | n > 0}))]
pub fn test1(vec: &mut RVec<i32>) {
    vec[1] = 5; //~ ERROR refinement type
}

#[flux::sig(fn({&mut RVec<i32>[@n] | n > 0}))]
pub fn test2(vec: &mut RVec<i32>) {
    vec[1] = 5; //~ ERROR refinement type
}
