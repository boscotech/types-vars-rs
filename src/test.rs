pub fn print_type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

use super::*;

#[test]
fn grading() {
    let answers = main();
    assert_eq!(print_type_of(&answers.0), "(f64, f64)".to_string());
    assert_eq!(print_type_of(&answers.1), "(f64, f64)".to_string());
    assert_eq!(print_type_of(&answers.2), "(f64, f64)".to_string());
    assert_eq!((answers.1.0 + answers.2.0) / 2.0, answers.0.0);
    assert_eq!((answers.1.1 + answers.2.1) / 2.0, answers.0.1);
}