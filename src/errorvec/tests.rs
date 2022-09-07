use crate::ErrorVec;
use indoc::indoc;

#[test]
fn display() {
    use std::io::{Error, ErrorKind::Other};

    let ev: ErrorVec<Error> = [
        std::fs::read_to_string("/_%!@_SHOULD_NOT_EXIST")
            .err()
            .unwrap(),
        Error::new(Other, "something borked".to_string()),
    ]
    .into_iter()
    .collect();

    assert_eq!(
        &ev.to_string(),
        indoc! { r#"
            [error 1 of 2] No such file or directory (os error 2)

            [error 2 of 2] something borked
        "# },
    );
}
