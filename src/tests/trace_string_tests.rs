use crate::{TraceString, format};

#[test]
fn create_string() {
    let string = TraceString::new();
    assert_eq!(string.length, 0);
    for instance in string.buffer.into_iter() {
        assert_eq!(instance, 0);
    }
}

#[test]
fn format_string() {
    let string = format(format_args!("Hello {}!", "World"));
    const RES: &str = "Hello World!";

    assert_eq!(string.length, RES.len());
    for i in 0..usize::min(string.length, RES.len()) {
        assert_eq!(string.buffer[i], RES.as_bytes()[i]);
    }
}
