use std::io;
use std::str::FromStr;

extern crate nom;
use nom::bytes::complete::tag_no_case;
use nom::bytes::streaming::take_while;
use nom::character::complete::{not_line_ending, space0};
use nom::character::is_space;
use nom::combinator::opt;
use nom::error::Error as NomError;
use nom::error::ErrorKind as NomErrorKind;
use nom::error::FromExternalError;
use nom::sequence::preceded;

use super::{AuthMech, IMAPStore, IMAPStoreBuilder};

#[derive(Debug, PartialEq)]
pub enum IMAPStoreFields {
    Host(String),
    AuthMechs(Vec<AuthMech>),
}

pub fn from_mbsync(content: String) -> Result<(), io::Error> {
    Ok(())
}

// return a parser for a line of the form "Host XYZ"
fn with_preceding_tag<'a>(t: &'a str) -> impl FnMut(&'a str) -> nom::IResult<&'a str, &'a str> {
    preceded(tag_no_case(t), preceded(space0, not_line_ending))
}

fn host(i: &str) -> nom::IResult<&str, IMAPStoreFields> {
    let (i, h) = with_preceding_tag("Host")(i)?;
    Ok((i, IMAPStoreFields::Host(h.to_owned())))
}

fn auth_mechs(i: &str) -> nom::IResult<&str, IMAPStoreFields> {
    let (i, a) = preceded(tag_no_case("AuthMechs"), preceded(space0, not_line_ending))(i)?;
    let a2 = AuthMech::from_str(a).map_err(|e| {
        nom::Err::Error(nom::error::Error {
            input: a,
            code: nom::error::ErrorKind::OneOf,
        })
    })?;
    Ok((i, IMAPStoreFields::AuthMechs(vec![a2])))
}

fn store_imap(stanza: &str) -> nom::IResult<&str, IMAPStore> {
    let store = IMAPStoreBuilder::default()
        .name("potato")
        .host("mashed")
        .map_err(|e| {
            nom::Err::Failure(NomError::from_external_error(
                stanza,
                NomErrorKind::Alpha,
                e,
            ))
        })?
        .add_auth_mech(&AuthMech::Login)
        .user("user")
        .pass("pwd")
        .build()
        .map_err(|e| {
            nom::Err::Failure(NomError::from_external_error(
                stanza,
                NomErrorKind::Alpha,
                e,
            ))
        })?;
    Ok((stanza, store))
}

mod test {
    use crate::config::{AuthMech, CmdField, SSLType};

    use super::*;

    #[test]
    fn test_parse_host() {
        let stanza = r#"Host imap.gmail.com"#;
        let res = host(stanza);
        assert_eq!(
            res,
            Ok(("", IMAPStoreFields::Host("imap.gmail.com".to_string())))
        );
    }

    #[test]
    fn test_parse_ssltype() {
        let stanza = r#"SSLType IMAPS"#; // or STARTTLS, or not present
        let res = host(stanza);
        assert_eq!(
            res,
            Ok(("", IMAPStoreFields::Host("imap.gmail.com".to_string())))
        );
    }

    #[test]
    fn test_parse_authmechs() {
        let stanza0 = r#"AuthMechs LOGIN"#;

        let res0 = auth_mechs(stanza0);
        assert_eq!(
            res0,
            Ok(("", IMAPStoreFields::AuthMechs(vec![AuthMech::Login])))
        );

        let stanza1 = r#"AuthMechs PLAIN"#;
        let res1 = auth_mechs(stanza1);
        assert_eq!(
            res1,
            Ok(("", IMAPStoreFields::AuthMechs(vec![AuthMech::Plain])))
        );

        let stanza2 = r#"AuthMechs POTATO"#;
        let res2 = auth_mechs(stanza2);
        assert_eq!(
            res2,
            Err(nom::Err::Error(nom::error::Error {
                input: "POTATO",
                code: nom::error::ErrorKind::OneOf
            }))
        );
    }

    #[test]
    fn test_parse_user_usercmd() {
        let stanza = r#"User rami.chowdhury@gmail.com"#;
        let res = host(stanza);
        assert_eq!(
            res,
            Ok(("", IMAPStoreFields::Host("imap.gmail.com".to_string())))
        );
    }

    #[test]
    fn test_parse_pass_passcmd() {
        let stanza = r#"PassCmd "pass personal/rami.chowdhury@gmail.com""#;
        let res = host(stanza);
        assert_eq!(
            res,
            Ok(("", IMAPStoreFields::Host("imap.gmail.com".to_string())))
        );
    }

    #[test]
    fn test_parse_magic_comment_for_exec_action() {
        let stanza = r#"PassCmd "pass personal/rami.chowdhury@gmail.com""#;
        let res = host(stanza);
        assert_eq!(
            res,
            Ok(("", IMAPStoreFields::Host("imap.gmail.com".to_string())))
        );
    }

    #[test]
    fn test_parse_store_imap() {
        let stanza = r#"IMAPStore primary-remote
Host imap.gmail.com
SSLType IMAPS
AuthMechs LOGIN
User rami.chowdhury@gmail.com
PassCmd "pass personal/rami.chowdhury@gmail.com"
"#;
        let res = store_imap(stanza);
        match res {
            Ok((i, ref o)) => println!("i: {:?} | o: {:?}", i, o),
            _ => println!("error!"),
        }

        assert_eq!(
            res,
            Ok((
                stanza,
                IMAPStore::new(
                    "primary-remote",
                    "imap.gmail.com",
                    SSLType::IMAPS,
                    &[AuthMech::Login],
                    CmdField::RawValue("rami.chowdhury@gmail.com".to_string()),
                    CmdField::ExecCommand("pass personal/rami.chowdhury@gmail.com".to_string())
                )
                .unwrap()
            ))
        );
    }
}
