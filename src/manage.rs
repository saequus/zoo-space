pub mod index {
  
    #[allow(dead_code)]
    pub fn show_test() {
        println!("This is show_test from manage::index mod");
    }
    
    #[allow(dead_code)]
    pub fn vector_test() {
        let x = vec![1, 2, 10, 33];
        let y = vec![100, 2, 11];
        let z = merge_arrays(x, y);
        print_array(z);
    }

    #[allow(dead_code)]
    pub fn merge_arrays(v: Vec<i32>, m: Vec<i32>) -> Vec<i32> {
        let mut r: Vec<i32> = Vec::new();
        let shortest_len;

        if v.len() > m.len() {
            shortest_len = m.len();
        } else {
            shortest_len = v.len();
        }
        
        if shortest_len != 0 {
            for i in 0..shortest_len {
                r.push(v[i] + m[i]); 
            }

            if v.len() > m.len() {
                for i in m.len()..v.len() {
                    r.push(v[i]);
                }
            } else {
                for i in v.len()..m.len()  {
                    r.push(m[i]); 
                }
            }
        } 
        r 
    }

    #[allow(dead_code)]
    pub fn print_array(v: Vec<i32>) {
        for i in 0..v.len() {
            println!("Item {} value {}", i, v[i]);
        } 
    }
}

#[cfg(test)]
mod tests {
    use crate::manage::index;    
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_merge_arrays_first_array_longer() {
        let x = vec![1, 2, 3, 4];
        let y = vec![1, 2, 3];
        
        
        let merged = index::merge_arrays(x, y);
        assert_eq!(merged, vec![2, 4, 6, 4]);
    }

   #[test]
    fn test_merge_arrays_second_array_longer() {
        let x = vec![1, 2];
        let y = vec![1, 2, 3, 4];
        
        
        let merged = index::merge_arrays(x, y);
        assert_eq!(merged, vec![2, 4, 3, 4]);
    }     
}

