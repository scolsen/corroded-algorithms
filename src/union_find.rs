// union-find algorithms

/// The quick-find algorithm.
pub fn quick_find(p: usize, q: usize) -> (i32, i32){
  // Initialize a vector of integers of a fixed size.
  let mut id: Vec<i32> = (0..10000).collect();
  
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

/// The quick-union algorithm.
pub fn quick_union(p: usize, q: usize) -> (i32, i32) {
  let mut id: Vec<i32> = (0..10000).collect();

  let result = loop {
    let mut i: usize = p;
    let mut j: usize = q;
    
    while i != id[i] as usize {
      i = id[i] as usize; 
    }
    
    while j != id[j] as usize {
      j = id[j] as usize;     
    }

    if i == j {
      break (id[p], id[q]);     
    }

    id[i] = j as i32;
  };

  result
}
