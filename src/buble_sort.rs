pub fn run(mut test_array: [i32; 6]) {
    let test_array_len = test_array.len() - 1;

    for _i in 0..test_array.len() {
        for _a in 0..test_array_len {
            if test_array[_a] > test_array[_a + 1] {
                let test_save = test_array[_a + 1];
                test_array[_a + 1] = test_array[_a];
                test_array[_a] = test_save;
            }
        }
    }

    println!("{:?}", test_array);
}
