use linear_regression::linear_regression::simple_linear_regression;


fn main() {
    let data = vec!{(1.0, 1.0), (1.2,1.24), (3.2,4.0), (4.0,3.9)};
    let line = simple_linear_regression(&data, &2, &1000, 0.04);
    println!("bias of {}, weight of {}", line.bias, line.weight);
}


