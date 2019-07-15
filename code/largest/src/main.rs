fn largest_i(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn largest_f(list: &[f32]) -> f32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn main() {
    let integers = vec![53, 12, 76, 19, 44, 98, 72];
    let floats = vec![1.45, 93.9, 64.3, 91.3, 45.1];

    println!("largest of {:?}: {}", integers, largest_i(&integers));
    println!("largest of {:?}: {}", floats, largest_f(&floats));

    println!("largest of {:?}: {}", integers, largest(&integers));
    println!("largest of {:?}: {}", floats, largest(&floats));
}
