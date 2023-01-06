use std::{thread, time};

fn parallel_map<T, U, F>(input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default + std::fmt::Display,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    output_vec.resize_with(input_vec.len(), Default::default);

    let (sender1, receiver1) = crossbeam_channel::unbounded();
    let (sender2, receiver2) = crossbeam_channel::unbounded();
    let mut threads = Vec::with_capacity(num_threads);

    for _ in 0..num_threads {
        let receiver1 = receiver1.clone();
        let sender2 = sender2.clone();
        threads.push(thread::spawn(move || {
            while let Ok((index, number)) = receiver1.recv() {
                sender2.send((index, f(number))).unwrap();
            }
        }));
    }
    drop(sender2);

    for (index, number) in input_vec.into_iter().enumerate() {
        sender1.send((index, number)).unwrap();
    }
    drop(sender1);

    for thread in threads {
        thread.join().unwrap();
    }

    while let Ok((index, number)) = receiver2.recv() {
        output_vec[index] = number;
    }

    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
