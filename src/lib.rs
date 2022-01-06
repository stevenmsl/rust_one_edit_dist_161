use std::cmp;

#[derive(Debug, PartialEq)]
pub struct Solution {}

/* key takeaways
   - you want to transform from s into t in one edit
   - if their length is the same you can only replace
     one char in s for it to become t. That means if
     there are more than one different char that it's
     not possible
   - if their length are different, once you detect
     the first difference you need to do one more
     check on the longer word right away
     - let say s is "abde" and t is "acbde"
       - so the second chars are different
       - you look ahead one more on t
         - it's the same as to make sure the remaining
           chars in s which are "bde" are
           the same as in t after the char 'c' which
           are also "bde"
*/

impl Solution {
  pub fn is_one_edit_distance(s: &String, t: &String) -> bool {
    let s = Self::to_chars(s);
    let t = Self::to_chars(t);

    let m = s.len();
    let n = t.len();

    let mut count: usize = 0;

    for i in 0..cmp::min(m, n) {
      let mut pos_s = i;
      let mut pos_t = i;

      /*
        - the first difference has been encountered
        - you need to look ahead one more on the long
          word
        - your goal is to make sure the remaining chars
          from both s and t are identical as you already
          detected one difference
      */
      if count > 0 {
        if m > n {
          pos_s += 1;
        } else if m < n {
          pos_t += 1;
        }
      }

      if s[pos_s] == t[pos_t] {
        continue;
      } else {
        count = count + 1;
        if count > 1 {
          //game over
          return false;
        }
        /* first time encountered the difference
           - if their length is different compare
             one more char
        */
        if m != n {
          if m > n {
            pos_s += 1;
          } else if m < n {
            pos_t += 1;
          }
          if s[pos_s] != t[pos_t] {
            return false;
          }
        }
      }
    }

    true
  }

  pub fn to_chars(s: &String) -> Vec<char> {
    s.to_ascii_lowercase().chars().collect()
  }

  pub fn test_fixture_1() -> (String, String) {
    (String::from("ab"), String::from("acb"))
  }

  pub fn test_fixture_2() -> (String, String) {
    (String::from("cab"), String::from("ad"))
  }

  pub fn test_fixture_3() -> (String, String) {
    (String::from("1203"), String::from("1213"))
  }

  pub fn test_fixture_4() -> (String, String) {
    (String::from("abde"), String::from("acbde"))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_1() {
    let (s, t) = Solution::test_fixture_1();
    assert_eq!(Solution::is_one_edit_distance(&s, &t), true);
  }

  #[test]
  fn sample_2() {
    let (s, t) = Solution::test_fixture_2();
    assert_eq!(Solution::is_one_edit_distance(&s, &t), false);
  }

  #[test]
  fn sample_3() {
    let (s, t) = Solution::test_fixture_3();
    assert_eq!(Solution::is_one_edit_distance(&s, &t), true);
  }
  #[test]
  fn sample_4() {
    let (s, t) = Solution::test_fixture_4();
    assert_eq!(Solution::is_one_edit_distance(&s, &t), true);
  }
}
