use crate::service::Service;
use crate::application::Application;
use crate::user::User;
use crate::usage::Usage;
use crate::errors::*;

use crate::ToParams;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Authorize,
    AuthRep,
    Report
}

type Extensions = HashMap<String, String>;

#[derive(Copy, Clone, Debug)]
pub struct ApiCall<'service, 'app, 'user, 'usage, 'extensions> {
    kind: Kind,
    service: &'service Service,
    application: &'app Application,
    user: Option<&'user User>,
    usage: Option<&'usage Usage>,
    extensions: Option<&'extensions Extensions>,
}

#[derive(Copy, Clone, Debug)]
pub struct Builder<'service, 'app, 'user, 'usage, 'extensions> {
    service: &'service Service,
    kind: Option<Kind>,
    application: Option<&'app Application>,
    user: Option<&'user User>,
    usage: Option<&'usage Usage>,
    extensions: Option<&'extensions Extensions>,
}

// TODO: we can improve this with a state machine of types so that we are required to set svc, app,
// user and kind before being able to set (required) the usage to build the call
impl<'service, 'app, 'user, 'usage, 'extensions> Builder<'service, 'app, 'user, 'usage, 'extensions> {
    pub fn new(service: &'service Service) -> Self {
        Builder {
            service,
            kind: Default::default(),
            application: Default::default(),
            user: Default::default(),
            usage: Default::default(),
            extensions: Default::default()
        }
    }

    pub fn service(&mut self, s: &'service Service) -> &mut Self {
        self.service = s;
        self
    }

    pub fn kind(&mut self, t: Kind) -> &mut Self {
        self.kind = Some(t);
        self
    }

    pub fn app(&mut self, a: &'app Application) -> &mut Self {
        self.application = Some(a);
        self
    }

    pub fn user(&mut self, u: &'user User) -> &mut Self {
        self.user = Some(u);
        self
    }

    pub fn usage(&mut self, usage: &'usage Usage) -> &mut Self {
        self.usage = Some(usage);
        self
    }

    pub fn extensions(&mut self, extensions: &'extensions Extensions) -> &mut Self {
        self.extensions = Some(extensions);
        self
    }

    pub fn build(&self) -> Result<ApiCall> {
        let kind = self.kind.ok_or_else(|| { "kind error".to_string() })?;
        let app = self.application.ok_or_else(|| { "app error".to_string()})?;
        Ok(ApiCall::new(kind, self.service, app, self.user, self.usage, self.extensions))
    }
}

impl<'service, 'app, 'user, 'usage, 'extensions> ApiCall<'service, 'app, 'user, 'usage, 'extensions> {
    pub fn builder(service: &'service Service) -> Builder {
        Builder::new(service)
    }

    pub fn new(kind: Kind, service: &'service Service, application: &'app Application,
               user: Option<&'user User>, usage: Option<&'usage Usage>,
               extensions: Option<&'extensions Extensions>) -> Self {
        Self { kind, service, application, user, usage, extensions }
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn service(&self) -> &'service Service {
        self.service
    }

    pub fn application(&self) -> &'app Application {
        self.application
    }

    pub fn user(&self) -> Option<&'user User> {
        self.user
    }

    pub fn extensions(&self) -> Option<&'extensions Extensions> {
        self.extensions
    }

    pub fn params(&self) -> Vec<(&str, &str)> {
        let mut params: Vec<(&str, &str)> = Vec::new();

        self.to_params(&mut params);
        params
    }
}

impl<'k, 'v, E> ToParams<'k, 'v, E> for ApiCall<'_, '_, '_, '_, '_> where E: Extend<(&'k str, &'v str)> {
    fn to_params<'s: 'k + 'v>(&'s self, extendable: &mut E) {
        self.service.to_params(extendable);
        self.application.to_params(extendable);

        if let Some(user_params) = self.user.as_ref() {
            user_params.to_params(extendable);
        }

        if let Some(usage_params) = self.usage.as_ref() {
            usage_params.to_params(extendable);
        }
    }
}