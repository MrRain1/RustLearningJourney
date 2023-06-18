use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut number_vector: Vec<i32> = Vec::new();
    let mut rnd = rand::thread_rng();

    for _i in 1..(rnd.gen_range(2..=50)){
        number_vector.push(rnd.gen_range(1..=50));
    }
    number_vector.sort();
    println!("{:?}", number_vector);

    let vector_length = number_vector.len();
    if vector_length%2 != 0{
        println!("The median value is {}, length is odd", &number_vector[(vector_length/2)]);
    }
    else {
        let median = (&number_vector[vector_length/2]+&number_vector[(vector_length/2)-1])/2;
        println!("The median value is {}, length is even", median);
    }

    let mut number_recurrence = HashMap::new();
    for number in number_vector{
        let count = number_recurrence.entry(number).or_insert(0);
        *count+=1;
    }
    
    let mut mode: i32 = 0;
    let mut max: i32 = 0;
    for (key, value) in &number_recurrence{
        if *value > max{
            max = *value;
            mode = *key;
        }
    }
    println!("The mode is: {}", mode);

}