pub fn run(mut test_array: [i32; 6]) {
    // Getting safe count of array, because if we don't then we will go out of range
    let test_array_len = test_array.len() - 1;

    // Looping into test_array until sorting is done
    for _i in 0..test_array.len() {
        // Looping in safe length
        for _a in 0..test_array_len {
            // Swapping elements if one we are on is grater than the next one
            if test_array[_a] > test_array[_a + 1] {
                let test_save = test_array[_a + 1];
                test_array[_a + 1] = test_array[_a];
                test_array[_a] = test_save;
            }
        }
    }

    // Printing results
    for _b in test_array {
        println!("{}", _b);
    }
}

// I know my comments are useless