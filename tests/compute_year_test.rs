use mybnp::compute_year;

#[test]
fn test_same_month() {
    // Operation in February, releve in February
    assert_eq!(compute_year(2, 2, 2025), 2025);
}

#[test]
fn test_operation_before_releve_same_year() {
    // Operation in January, releve in February
    assert_eq!(compute_year(1, 2, 2025), 2025);
}

#[test]
fn test_operation_after_releve_same_year() {
    // Operation in March, releve in February (within 6 months)
    assert_eq!(compute_year(3, 2, 2025), 2025);
}

#[test]
fn test_december_operation_january_releve() {
    // Operation in December, releve in January -> previous year
    assert_eq!(compute_year(12, 1, 2025), 2024);
}

#[test]
fn test_december_operation_february_releve() {
    // Operation in December, releve in February -> previous year
    assert_eq!(compute_year(12, 2, 2025), 2024);
}

#[test]
fn test_november_operation_january_releve() {
    // Operation in November, releve in January -> previous year (diff > 6)
    assert_eq!(compute_year(11, 1, 2025), 2024);
}

#[test]
fn test_october_operation_january_releve() {
    // Operation in October, releve in January -> previous year (diff > 6)
    assert_eq!(compute_year(10, 1, 2025), 2024);
}

#[test]
fn test_august_operation_january_releve() {
    // Operation in August, releve in January -> previous year (diff = 7 > 6)
    assert_eq!(compute_year(8, 1, 2025), 2024);
}

#[test]
fn test_july_operation_january_releve() {
    // Operation in July, releve in January -> same year (diff = 6, not > 6)
    assert_eq!(compute_year(7, 1, 2025), 2025);
}

#[test]
fn test_june_operation_december_releve() {
    // Operation in June, releve in December -> same year (op < releve)
    assert_eq!(compute_year(6, 12, 2025), 2025);
}
