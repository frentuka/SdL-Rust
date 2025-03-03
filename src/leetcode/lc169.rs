/*
Given an array nums of size n, return the majority element.

The majority element is the element that appears more than ⌊n / 2⌋ times.
You may assume that the majority element always exists in the array.
 */

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut actual = nums[0];
        let mut points = 0;

        for num in nums {
            if points == 0 {
                actual = num;
            }

            if actual == num {
                points += 1;
            } else {
                points -= 1;
            }
        }

        actual
    }
}