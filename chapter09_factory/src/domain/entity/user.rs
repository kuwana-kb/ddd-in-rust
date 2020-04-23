#![allow(dead_code)]
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use super::super::{Name, UserId};

#[derive(Clone, Debug, Getters, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: Name,
}

impl User {
    // インスタンスの生成がファクトリに移設されたため、Userをインスタンス化する際には必ず外かからUserIdが引き渡される
    // したがってコンストラクタは1つで済む
    // TODO: pubの範囲をFactoryに限定したい
    pub fn new(id: UserId, name: Name) -> Self {
        Self { id, name }
    }

    // 以下コンストラクタは不要となる
    // pub fn new(name: Name) -> Self {
    //     Self {
    //         id: UserId::default(),
    //         name,
    //         mail_address,
    //     }
    // }

    pub fn change_name(&mut self, name: Name) {
        self.name = name;
    }
}
