fn main() {
    let mut mutable_vec = vec![1, 2, 3, 4, 5];

    println!("mutable vec before transform: {:?}", mutable_vec);

    transform(&mut mutable_vec, |i| (*i) * 2);

    println!("mutable vec after transform: {:?}", mutable_vec);

    let immutable_vec = vec![1,2,3,4,5];

    println!("immutable vec before map: {:?}", immutable_vec);

    let immutable_vec_doubled = map(&immutable_vec, |i| *i * 2);

    println!("immutable vec after map: {:?}", immutable_vec);
    println!("immutable vec doubled: {:?}", immutable_vec_doubled);
}

fn transform<T>(v: &mut Vec<T>, mapper: fn(&T) -> T) {
    for element in v.iter_mut() {
        *element = mapper(element);
    }
}

fn map<T>(v: &Vec<T>, mapper: fn(&T) -> T) -> Vec<T> {
    let mut new = Vec::with_capacity(v.len());

    for element in v.iter() {
        new.push(mapper(element));
    }

    new
}
