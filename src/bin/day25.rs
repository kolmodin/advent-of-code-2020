fn transform_steps(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % 20201227;
    }
    value
}

fn transform_find_loop(subject_number: usize, final_value: usize) -> usize {
    let mut value = 1;
    for i in 0.. {
        if value == final_value {
            return i;
        }
        value = (value * subject_number) % 20201227;
    }
    panic!("unreachable")
}

fn main() {
    assert_eq!(transform_steps(7, 8), 5764801);
    assert_eq!(transform_steps(7, 11), 17807724);
    assert_eq!(transform_steps(17807724, 8), 14897079);
    assert_eq!(transform_steps(5764801, 11), 14897079);

    assert_eq!(transform_find_loop(7, 5764801), 8);
    assert_eq!(transform_find_loop(7, 17807724), 11);

    let card_public_key = 6929599;
    let door_public_key = 2448427;

    let card_loop_size = transform_find_loop(7, card_public_key);
    let encryption_key = transform_steps(door_public_key, card_loop_size);

    println!("encryption key: {}", encryption_key);
}
