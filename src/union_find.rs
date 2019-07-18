// union-find algorithms
/// The quick-find algorithm.
pub fn quick_find(p: usize, q: usize) -> (i32, i32){
  // Initialize a vector of integers of a fixed size.
  let mut id: Vec<i32> = vec![0; 10000];
  
  // Loop over each element of the array, setting it equivalent to its index.
  for n in 0..10000 {
    id[n] = n as i32; 
  }
  
  let result = loop {
    if id[p] == id[q] {
      break (id[p], id[q]);
    }
    let t: i32 = id[p];
    for n in 0..10000 {
      if id[n] == t {
        id[n] = id[q];
      }  
    }
  };
  
  result
}
