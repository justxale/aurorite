pub fn create_request_id() -> String {
    let mut request_id = ['0'; 8];
    for c in &mut request_id {
        *c = fastrand::alphanumeric();
    }
    request_id.into_iter().collect()
}