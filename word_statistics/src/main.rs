use std::collections::HashMap;


fn mean(vec: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for &item in vec {
        sum += item;
    }
    sum / vec.len() as i64
}

fn median(vec: &Vec<i64>) -> i64 {
    let mut vec_local = vec.to_vec();
    vec_local.sort();
    println!("vec sorted: {:?}", vec_local);
    
    match vec.len() % 2 {
        0 => {
            let mid_index_second = vec.len() / 2;
            let mid_index_first = mid_index_second - 1;
            (vec_local[mid_index_first] + vec_local[mid_index_second]) / 2
        }
        _ => {
            let mid_index = vec.len() / 2;
            vec_local[mid_index]
        },
    }
}

fn mode_one(vec: &Vec<i64>) -> (u64, i64) {
    let mut counts = HashMap::new();
    
    for &item in vec {
        let freq = counts.entry(item).or_insert(0);
        *freq += 1;
    }

    let mut most_freq_key = 0;
    let mut max_freq = 0;
    for (key, freq) in &counts {
        if *freq > max_freq {
            max_freq = *freq;
            most_freq_key = *key;
        }
    }

    println!("{:#?}", counts);

    (max_freq, most_freq_key)
}

fn mode_all(vec: &Vec<i64>) -> (u64, Vec<i64>) {
    let mut counts = HashMap::new();
    
    for &item in vec {
        let freq = counts.entry(item).or_insert(0);
        *freq += 1;
    }

    let mut most_freq_keys = vec![];
    let mut max_freq = 0;
    for (key, freq) in &counts {
        if *freq == max_freq {
            most_freq_keys.push(*key);
        }
        if *freq > max_freq {
            max_freq = *freq;
            most_freq_keys.clear();
            most_freq_keys.push(*key);
        }
    }

    (max_freq, most_freq_keys)
}

fn main() {
    let sample = vec![209, 106, -232, 30, 215, -156, 105, -52, 30, 106];

    println!("sample len: {}", sample.len());
    println!("mean: {}", mean(&sample));
    println!("median: {}", median(&sample));
    println!("mode (one): {:?}", mode_one(&sample));
    println!("mode (all): {:?}", mode_all(&sample));
}
