use std::sync::{Arc, Mutex};

use crate::models::{hustle::Hustle, transaction::Transaction, user::User};

#[derive(Clone, Default)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub hustles: Arc<Mutex<Vec<Hustle>>>,
    pub transactions: Arc<Mutex<Vec<Transaction>>>,
}
