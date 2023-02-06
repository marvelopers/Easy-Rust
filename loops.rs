fn main() {
    let mut counter = 0;
    let mut counter2 = 0;

    'first_loop: loop {
        counter += 1;
        println!("The counter is: {}", counter);

        if (counter > 9) {
            println!("Now entering the second loop");

            'second_loop: loop {
                println!("The second counter is: {}", counter2);
                counter2 += 1;
                if (counter2 == 3) {
                    break 'first_loop;
                }
            }
        }
    }
}

// Range
fn range_main() {
    // 0..3 exculsive Range || 0..3= inculsive Range
    for number in 0..3 {
        println!("The number is {}", number);
    }
}

fn break_main() {
    let mut counter = 5;

    let my_number = loop {
        counter += 1;
        if counter % 53 == 3 {
            break counter;
        }
    };

    println!("my_number is now: {}", my_number);
}
