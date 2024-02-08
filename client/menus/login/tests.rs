#![cfg(test)]
use super::login::is_valid_email;
#[test]
fn test_mails() {
    //all of these examples are taken from https://en.wikipedia.org/wiki/Email_address
    let valid_mails = [
        "simple@example.com",
        "very.common@example.com",
        "x@example.com",
        "long.email-address-with-hyphens@and.subdomains.example.com",
        "user.name+tag+sorting@example.com",
        "name/surname@example.com",
        "admin@example",
        "example@s.example",
        "\" \"@example.org",
        "\"john..doe\"@example.org",
        "mailhost!username@example.org",
        "\"very.(),:;<>[]\\\".VERY.\\\"very@\\\\ \\\"very\\\".unusual\"@strange.example.com",
        "user%example.com@example.org",
        "user-@example.org",
        "postmaster@[123.123.123.123]",
        "postmaster@[IPv6:2001:0db8:85a3:0000:0000:8a2e:0370:7334]",
        "_test@[IPv6:2001:0db8:85a3:0000:0000:8a2e:0370:7334]",
    ];
    let invalid_mails = [
        "abc.example.com",
        "a@b@c@example.com",
        "a\"b(c)d,e:f;g<h>i[j\\k]l@example.com",
        "just\"not\"right@example.com",
        "this is\"not\\allowed@example.com",
        "this\\ still\\\"not\\\\allowed@example.com",
        "1234567890123456789012345678901234567890123456789012345678901234+x@example.com",
        "i.like.underscores@but_they_are_not_allowed_in_this_part",
    ];

    for mail in valid_mails {
        assert!(is_valid_email(mail), "mail {mail} should be marked as valid");
    }

    for mail in invalid_mails {
        assert!(!is_valid_email(mail), "mail {mail} should be marked as invalid");
    }
}
