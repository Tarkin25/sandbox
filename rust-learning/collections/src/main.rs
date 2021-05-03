use collections::LinkedList;

fn main() {

    let mut list = LinkedList::new();

    list.add(1)
        .add(2)
        .add(3)
        .add(4)
        .add(5)
        .for_each(|value| println!("{}", value));

}