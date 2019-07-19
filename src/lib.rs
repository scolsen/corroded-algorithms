mod union_find;

#[cfg(test)]
mod tests {
  use crate::union_find;

  #[test]
  fn it_works() {
      assert_eq!(2 + 2, 4);
  }

  // union-find tests
  #[test]
  fn test_quick_find() {
    assert_eq!((4, 4), union_find::quick_find(3, 4))    
  }
  
  #[test]
  fn test_quick_union() {
   assert_eq!((4, 4), union_find::quick_union(3, 4))    
  }

  #[test]
  fn test_weighted_quick_union() {
    assert_eq!((3, 3), union_find::weighted_quick_union(3, 4))   
  }
}
