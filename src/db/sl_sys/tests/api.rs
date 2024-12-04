use sl_sys::get_connection;

#[test]
fn test_get_connection() {
    if let Ok(_conn) = get_connection() {
        assert!(true);
    } else {
        assert!(false);
    }
}