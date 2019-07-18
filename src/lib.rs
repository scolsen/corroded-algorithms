mod union_find;

#[cfg(test)]
mod tests {
  use crate::union_find;

  #[test]
  fn it_works() {
      assert_eq!(2 + 2, 4);
  }

 // union-find
 #[test]
 fn test_quick_find() {
   assert_eq!((4, 4), union_find::quick_find(3, 4))    
 }
}
