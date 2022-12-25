use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);

    for i in [1, 1, 4, 5, 1, 4, 1, 9, 1, 9] {
        list.push_front(i);
    }

    let list2 = list.clone();

    println!("list2:\t{}", list2);

    println!("TEST: {}", list == list2);

    println!("list:\t{}", list);
    println!("list::size:\t{}", list.get_size());
    println!("list::top_element:\t{}", list.pop_front().unwrap());
    println!("list:\t{}", list);
    println!("list::size:\t{}", list.get_size());
    // println!("list:\t{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    for val in list {
        print!("{}, ", val);
    }
    println!();
    // println!("{}", list);
}
