use ulid::Ulid;

#[derive(Debug)]
pub enum Service {
    Local,
    GitHub
}

pub struct UserSource<'a> {
    pub id: usize,
    pub domain: &'a str,
    pub authorised: bool,
    pub banned: bool
}

#[derive(Debug)]
pub struct Login<'a> {
    pub service: Service, // The service that the user is using for auth
    pub service_user: &'a str, // User ID in the service provider
    pub uid: Ulid // User's ID that the login belongs to
}

impl Service {
    pub fn as_str(&self) -> &str {
        match self {
            Service::Local => "Local",
            Service::GitHub => "GitHub"
        }
    }
}