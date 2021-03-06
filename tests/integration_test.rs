use finances;

#[test]
fn validate_id_portugal_valid_test() {
    assert!(finances::validate_id("14349483 0 ZV3", finances::country::Code::PT));
}

#[test]
fn validate_id_portugal_invalid_test() {
    assert!(!finances::validate_id("11234455677890", finances::country::Code::PT));
}

#[test]
fn validate_id_france_valid_test() {
    assert!(finances::validate_id("2820819398814 09", finances::country::Code::FR));
}

#[test]
fn validate_id_france_invalid_test() {
    assert!(!finances::validate_id("123X123X123dDAS", finances::country::Code::FR));
}

#[test]
fn extract_france_test() {
    let citizen = finances::extract_information("2820819398814 09", finances::country::Code::FR).unwrap();
    assert_eq!(citizen.gender, 'F');
    assert_eq!(citizen.year_of_birth, 1982);
    assert_eq!(citizen.month_of_birth.unwrap(), 8);
    assert_eq!(citizen.place_of_birth.unwrap(), "Corrèze");
}
