pub fn run(mut test_array: [i32; 6]) {
    // Getting safe count of array
    let test_array_len = test_array.len() - 1;

    // Looping into test_array until sorting is done
    for _i in 0..test_array.len() {
        // Sorting loop
        for _a in 0..test_array_len {
            if test_array[_a] > test_array[_a + 1] {
                let test_save = test_array[_a + 1];
                test_array[_a + 1] = test_array[_a];
                test_array[_a] = test_save;
            }
        }
    }

    // Printing results
    println!("{:?}", test_array);
}

// I know my comments are useless