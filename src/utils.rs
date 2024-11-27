pub fn random_choice<T>(choices: &[T]) -> &T {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    choices.choose(&mut rng).unwrap()
}

