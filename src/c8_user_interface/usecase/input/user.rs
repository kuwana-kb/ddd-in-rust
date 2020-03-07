use derive_getters::Getters;
use derive_new::new;
use serde::Deserialize;

use crate::c8_user_interface::domain::{MailAddress, Name, UserId};

#[derive(Clone, Debug, Getters, new, Deserialize)]
pub struct CreateUserCommand {
    name: Name,
    mail_address: MailAddress,
}

// Updateメソッドのパラメータ群。Optionで定義したフィールドは任意の更新項目とする。
#[derive(Clone, Debug, Getters, new)]
pub struct UpdateUserCommand {
    id: UserId,
    name: Option<Name>,
    mail_address: Option<MailAddress>,
}

#[derive(Clone, Debug, Getters, new)]
pub struct DeleteUserCommand {
    id: UserId,
}
