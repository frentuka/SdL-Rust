/*
Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.
 */

struct Solution;

impl Solution {

    /*
    ej:

    [1,2,3,4,5,6,7]
    k = 3

    primer paso: invertir toda la lista
    [7,6,5,4,3,2,1]

    segundo paso: invertir de 0 hacia k (0..2)
    [5,6,7 , 4,3,2,1]

    tercer paso: invertir de k (inclusive) (3..=6)
    [5,6,7 , 1,2,3,4]
     */

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();

        // invierte toda la lista
        nums.reverse();

        // desde 0 hacia k, invierte esos items
        nums[..k].reverse();

        // desde k (incl) hasta nums.len-1, invierte de nuevo
        nums[k..].reverse();
    }

}