use derive_getters::Getters;
use derive_new::new;

use crate::c8_user_interface::{domain::UserId, MailAddress, Name};

// Updateメソッドのパラメータ群。Optionで定義したフィールドは任意の更新項目とする。
#[derive(Clone, Debug, Getters, new)]
pub struct UserUpdateCommand {
    id: UserId,
    name: Option<Name>,
    mail_address: Option<MailAddress>,
}

#[derive(Clone, Debug, Getters, new)]
pub struct UserDeleteCommand {
    id: UserId,
}
