pub fn solution() {
    println!("Solution for 2105");
    let plants = vec![2, 2, 3, 3];

    let num_refills = minimum_refill(plants, 5, 5);

    println!("{}", format!("{:?}", num_refills));
}

pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
    let mut a_bucket = Bucket {
        start_capacity: capacity_a,
        current_capacity: capacity_a,
        index: 0,
        refills: 0,
    };

    let mut b_bucket = Bucket {
        start_capacity: capacity_b,
        current_capacity: capacity_b,
        index: plants.len() - 1,
        refills: 0,
    };
    loop {
        // this means that they have swapped places
        if a_bucket.index > b_bucket.index {
            break;
        }
        // the buckets are at the same index
        if a_bucket.index == b_bucket.index {
            if a_bucket.current_capacity > b_bucket.current_capacity {
                a_bucket.water_plant(plants[a_bucket.index]);
            } else {
                b_bucket.water_plant(plants[b_bucket.index]);
            }
        } else {
            a_bucket.water_plant(plants[a_bucket.index]);
            b_bucket.water_plant(plants[b_bucket.index]);
        }

        // break if index is going to be less than 0
        if b_bucket.index == 0 {
            break;
        }

        a_bucket.index += 1;
        b_bucket.index -= 1;
    }

    a_bucket.refills + b_bucket.refills 
}

struct Bucket {
    start_capacity: i32,
    current_capacity: i32,
    index: usize,
    refills: i32,
}

impl Bucket {
    pub fn water_plant(&mut self, plant_water_unit: i32) {
        if self.current_capacity >= plant_water_unit {
            self.current_capacity -= plant_water_unit
        } else {
            // we need to refil first
            self.current_capacity = self.start_capacity;
            if self.current_capacity < plant_water_unit {
                panic!("there's no way to water this plant")
            }
            self.current_capacity -= plant_water_unit;
            self.refills += 1;
        }
    }
}
