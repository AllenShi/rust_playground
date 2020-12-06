use super::conversionexam::Number;

#[test]
fn unit_test_number() {
    let i = Number::from(20);
    println!("The number is {:?} firstly", i);
    println!("The number is {:?} secondly", i);
}