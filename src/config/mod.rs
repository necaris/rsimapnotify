use url::Host;

struct Config {}

#[derive(Debug, Clone, PartialEq)]
pub enum SSLType {
    // None - no security. This is the default when Tunnel is set, as tunnels are usually secure.
    NoSecurity,
    // STARTTLS - security is established via the STARTTLS extension after connecting the regular IMAP port 143. Most servers support this, so it is the default (unless a tunnel is used).
    StarTLS,
    // IMAPS - security is established by starting SSL/TLS negotiation right after connecting the secure IMAP port 993.
    IMAPS,
}

// https://www.iana.org/assignments/sasl-mechanisms/sasl-mechanisms.xhtml#sasl-mechanisms-1
#[derive(Debug, Clone, PartialEq, strum_macros::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum AuthMech {
    Plain,
    Login,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CmdField {
    RawValue(String),
    ExecCommand(String),
}

#[derive(PartialEq, Debug)]
pub struct IMAPStore {
    name: String,
    host: Host,
    ssl_type: SSLType,
    auth_mechs: Vec<AuthMech>,
    user: CmdField,
    password: CmdField,
}

impl IMAPStore {
    pub fn new(
        name: &str,
        host: &str,
        ssl_type: SSLType,
        auth_mechs: &[AuthMech],
        user: CmdField,
        password: CmdField,
    ) -> Result<Self, String> {
        let mut builder = &mut IMAPStoreBuilder::default();
        builder = builder.name(name);
        builder = builder
            .host(host)
            .map_err(|_| format!("Couldn't parse host from {}", host).to_owned())?;
        builder = builder.ssl_type(ssl_type);

        for am in auth_mechs {
            builder.add_auth_mech(am);
        }
        builder = match user {
            CmdField::RawValue(v) => builder.user(&v),
            CmdField::ExecCommand(v) => builder.user_cmd(&v),
        };
        builder = match password {
            CmdField::RawValue(v) => builder.pass(&v),
            CmdField::ExecCommand(v) => builder.pass_cmd(&v),
        };
        builder.build()
    }
}

#[derive(PartialEq, Debug)]
pub struct IMAPStoreBuilder {
    name: Option<String>,
    host: Option<Host>,
    ssl_type: Option<SSLType>,
    auth_mechs: Vec<AuthMech>,
    user: Option<CmdField>,
    password: Option<CmdField>,
}

impl Default for IMAPStoreBuilder {
    fn default() -> Self {
        IMAPStoreBuilder {
            name: None,
            host: None,
            ssl_type: None,
            auth_mechs: Vec::new(),
            user: None,
            password: None,
        }
    }
}

impl IMAPStoreBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn host(&mut self, host: &str) -> Result<&mut Self, url::ParseError> {
        self.host = Some(Host::parse(host)?);
        Ok(self)
    }

    pub fn ssl_type(&mut self, ssl_type: SSLType) -> &mut Self {
        self.ssl_type = Some(ssl_type);
        self
    }

    pub fn add_auth_mech(&mut self, mech: &AuthMech) -> &mut Self {
        self.auth_mechs.push(mech.clone());
        self
    }

    pub fn user(&mut self, value: &str) -> &mut Self {
        self.user = Some(CmdField::RawValue(value.to_owned()));
        self
    }

    pub fn user_cmd(&mut self, value: &str) -> &mut Self {
        self.user = Some(CmdField::ExecCommand(value.to_owned()));
        self
    }

    pub fn pass(&mut self, value: &str) -> &mut Self {
        self.password = Some(CmdField::RawValue(value.to_owned()));
        self
    }

    pub fn pass_cmd(&mut self, value: &str) -> &mut Self {
        self.password = Some(CmdField::ExecCommand(value.to_owned()));
        self
    }

    pub fn build(&self) -> Result<IMAPStore, String> {
        if self.name.is_none() {
            return Err("name is required".to_owned());
        }
        if self.host.is_none() {
            return Err("host is required".to_owned());
        }
        if self.ssl_type.is_none() {
            return Err("ssl_type is required".to_owned());
        }
        if self.auth_mechs.is_empty() {
            return Err("auth_mechs are required".to_owned());
        }
        if self.user.is_none() {
            return Err("user or user_cmd is required".to_owned());
        }
        if self.password.is_none() {
            return Err("pass or pass_cmd is required".to_owned());
        }

        Ok(IMAPStore {
            name: self.name.as_ref().unwrap().clone(),
            host: self.host.as_ref().unwrap().clone(),
            ssl_type: self.ssl_type.as_ref().unwrap().clone(),
            auth_mechs: self.auth_mechs.to_vec(),
            user: self.user.as_ref().unwrap().clone(),
            password: self.password.as_ref().unwrap().clone(),
        })
    }
}

struct MaildirStore {}

pub struct Channel {}

struct Group {}

pub mod formatter;
pub mod parser;
