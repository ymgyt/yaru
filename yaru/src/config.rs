pub static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub mod github {
    /// Github OAuth application client id
    pub static CLIENT_ID: &str = "e6f0c207fb1ccb20865e";
}
