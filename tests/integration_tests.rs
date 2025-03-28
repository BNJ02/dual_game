use dual_game::utils::calculate_score;

#[test]
fn test_calculate_score() {
    assert_eq!(calculate_score(0, 0, 50), 150);
    assert_eq!(calculate_score(3, 0, 50), 130);
    assert_eq!(calculate_score(8, 1, 40), 50);
    assert_eq!(calculate_score(15, 1, 50), 45);
    assert_eq!(calculate_score(30, 2, 50), 23);
    assert_eq!(calculate_score(55, 0, 50), 50);
}
