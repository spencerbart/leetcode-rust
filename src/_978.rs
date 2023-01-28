pub fn solution() {
    println!("Solution for 978");

    // let arr = vec![0,1,1,0,1,0,1,1,0,0];
    let arr = vec![0,8,45,88,48,68,28,55,17,24];

    let max_size = max_turbulence_size(arr);

    println!("Output: {}", max_size);
}

pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
    let mut i = 0;
    let arr_size = arr.len() - 1;
    let mut max_size = 0;
    let mut running_max_size = 0;

    loop {
        if i == arr_size {
            if running_max_size > max_size {
                max_size = running_max_size;
            }
            break;
        } else if i % 2 == 0 || i == 0 {
            if arr[i] < arr[i + 1] {
                running_max_size += 1;
                // println!("{} < {} is true", arr[i], arr[i+1]); 
            } else {
                // println!("{} < {} is false", arr[i], arr[i+1]); 
                if running_max_size > max_size {
                    max_size = running_max_size;
                }
                running_max_size = 0;
            }
        } else {
            if arr[i] > arr[i + 1] {
                running_max_size += 1;
                // println!("{} > {} is true", arr[i], arr[i+1]); 
            } else {
                // println!("{} > {} is false", arr[i], arr[i+1]);
                if running_max_size > max_size {
                    max_size = running_max_size;
                }
                running_max_size = 0;
            }
        }

        // println!("index: {}\tnumber: {}\tmax_size: {}\trunning_max_size: {}", i, arr[i], max_size, running_max_size);

        i += 1;
    }

    running_max_size = 0;
    i = 0;

    loop {
        if i == arr_size {
            if running_max_size > max_size {
                max_size = running_max_size;
            }
            break;
        } else if i % 2 == 0 || i == 0 {
            if arr[i] > arr[i + 1] {
                running_max_size += 1;
                // println!("{} < {} is true", arr[i], arr[i+1]); 
            } else {
                // println!("{} < {} is false", arr[i], arr[i+1]); 
                if running_max_size > max_size {
                    max_size = running_max_size;
                }
                running_max_size = 0;
            }
        } else {
            if arr[i] < arr[i + 1] {
                running_max_size += 1;
                // println!("{} > {} is true", arr[i], arr[i+1]); 
            } else {
                // println!("{} > {} is false", arr[i], arr[i+1]);
                if running_max_size > max_size {
                    max_size = running_max_size;
                }
                running_max_size = 0;
            }
        }

        // println!("index: {}\tnumber: {}\tmax_size: {}\trunning_max_size: {}", i, arr[i], max_size, running_max_size);

        i += 1;
    }

    max_size + 1
}