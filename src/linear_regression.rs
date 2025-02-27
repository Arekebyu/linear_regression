use rand::Rng;

pub struct SimpleLine {
    pub weight: f64,
    pub bias: f64,
}
fn divide_partitions (data: &Vec<(f64,f64)>, max_size: &i32) -> Vec<(usize, usize)> {
    return vec!{(0,4)};
    let mut partitions: Vec<(usize, usize)> = Vec::new();
    let mut index: usize = 0;
    while index < data.len() {
        let new_index = index + rand::thread_rng().gen_range(1..*max_size) as usize;
        partitions.push((index, Ord::max(new_index, data.len())));
        index = new_index;
    }
    return partitions;
}

pub fn simple_linear_regression (data: &Vec<(f64, f64)>, batch: &i32, epoch: &i32, rate: f64) ->SimpleLine {
    let mut weight = 0.0;
    let mut bias = 0.0;
    for _ in 0..*epoch {
        let partitions: Vec<(usize, usize)> = divide_partitions(data, batch);
        for (lower, upper) in partitions {
            let mut weight_change = 0.0;
            let mut bias_change = 0.0;
            let n: f64 = (upper-lower) as f64;
            for index in lower..upper{
                let (x, y) = data[index];
                weight_change += (x * weight + bias - y) * 2.0 * x;
                bias_change += (x * weight + bias - y) * 2.0;
            }
            weight -= weight_change * rate / n ;
            bias -= bias_change * rate  / n;
        }
    }
    return SimpleLine {
        weight, bias
    }    
}
