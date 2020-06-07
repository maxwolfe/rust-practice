use std::collections::HashMap;

fn main() {
    let mut list_of_ints = vec![1, 2, 3, 4, 7, 6, 5, 4, 8, 2, 4, 9, 3, 1];

    list_of_ints.sort();

    println!("Mean of integers: {}", mean(&list_of_ints));
    println!("Median of integers: {}", median(&list_of_ints));
    println!("Mode of integers: {}", mode(&list_of_ints));
}

fn mean(list_of_ints: &Vec<i32>) -> f32 {
    let mut sum: i32 = 0;

    for num in list_of_ints {
        sum += *num;
    }

    sum as f32 / list_of_ints.len() as f32
}

fn median(list_of_ints: &Vec<i32>) -> i32 {
    list_of_ints[list_of_ints.len() / 2]
}

fn mode(list_of_ints: &Vec<i32>) -> i32 {
    let mut count: HashMap<i32, i32> = HashMap::new();

    for num in list_of_ints {
        let occurrence = count.entry(*num).or_insert(0);
        *occurrence += 1;
    }

    let mut maximum = (0, 0);

    for num in list_of_ints {
        match count.get(num) {
            Some(val) => {
                if *val > maximum.1 {
                    maximum = (*num, *val);
                }
            }
            None => (),
        }
    }

    maximum.0
}
