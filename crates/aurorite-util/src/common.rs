pub fn create_hex<const LEN: usize>() -> String {
    let mut request_id = ['0'; LEN];
    for c in &mut request_id {
        *c = fastrand::alphanumeric();
    }
    request_id.into_iter().collect()
}
