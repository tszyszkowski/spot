use std::rc::Rc;
use std::cell::RefCell;

use crate::app::{AppModel, AppAction, ActionDispatcher};
use crate::app::components::{LoginModel};
use crate::app::credentials;

pub struct LoginModelImpl {
    dispatcher: Box<dyn ActionDispatcher>
}

impl LoginModelImpl {
    pub fn new(dispatcher: Box<dyn ActionDispatcher>) -> Self {
        Self { dispatcher }
    }
}

impl LoginModel for LoginModelImpl {

    fn try_autologin(&self) -> bool {
        if let Ok(creds) = credentials::try_retrieve_credentials() {
            self.dispatcher.dispatch(AppAction::TryLogin(creds.username, creds.password));
            true
        } else {
            false
        }
    }

    fn login(&self, u: String, p: String) {
        self.dispatcher.dispatch(AppAction::TryLogin(u, p));
    }
}
