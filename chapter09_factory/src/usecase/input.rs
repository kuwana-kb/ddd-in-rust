use derive_getters::Getters;
use derive_new::new;
use serde::Deserialize;

use crate::domain::Name;

#[derive(Clone, Debug, Getters, new, Deserialize)]
pub struct CreateUserCommand {
    name: Name,
}
