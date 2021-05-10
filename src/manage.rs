pub mod index {
    pub fn show_test() {
        println!("This is show_test from manage::index mod");
    }

    pub fn vector_test() {
        let mut x = vec![1, 2, 10, 33];
        let mut y = vec![100, 2, 11];
        x = merge_arrays(x, y);
        print_array(x);
    }

    pub fn merge_arrays(v: Vec<i32>, m: Vec<i32>) -> Vec<i32> {
        let mut r: Vec<i32> = Vec::new();
        let mut short_len = 0;
        let mut diff_len = 0;
        if v.len() > m.len() {
            short_len = m.len();
            diff_len = v.len() - short_len;
        } else {
            short_len = v.len();
            diff_len = m.len() - short_len;
        }
        
        for i in 0..short_len {
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
        r 
    }

    pub fn print_array(v: Vec<i32>) {
        for i in 0..v.len() {
            println!("Item {} value {}", i, v[i]);
        }
    }

    pub fn print_array_len(v: Vec<i32>) { 
        let length = v.len();
        println!("{} - length", length);
    }

}


