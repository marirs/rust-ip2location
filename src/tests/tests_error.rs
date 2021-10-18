use crate::error::Error;

#[test]
fn test_error_display() {
    assert_eq!(
        format!(
            "{:?}",
            Error::IoError(
                "Error opening DB file: No such file or directory (os error 2)".to_string()
            )
        ),
        "IoError: Error opening DB file: No such file or directory (os error 2)".to_string()
    );

    assert_eq!(
        format!("{:?}", Error::GenericError("an error occurred".to_string())),
        "GenericError: an error occurred".to_string()
    );

    assert_eq!(
        format!("{:?}", Error::RecordNotFound),
        "RecordNotFound: no record found".to_string()
    );

    assert_eq!(
        format!(
            "{:?}",
            Error::InvalidIP("ip address is invalid".to_string())
        ),
        "InvalidIP: ip address is invalid".to_string()
    );
}
