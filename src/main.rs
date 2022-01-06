use rust_one_edit_dist_161::Solution;

fn main() {
    let (s, t) = Solution::test_fixture_3();
    println!("{:?}", Solution::is_one_edit_distance(&s, &t));
}
