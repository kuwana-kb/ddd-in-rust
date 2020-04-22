#![allow(dead_code)]

use std::marker::PhantomData;
use std::ops::Add;

use rust_decimal::Decimal;

// 振る舞いを持つVO
// 具体的には通貨単位が一致した場合に限り加算が可能
//
// このケースでは通貨単位を型として表現している
// Money<T>のTで通貨単位を表すようにする
// ここで嬉しいのは、誤った通貨単位同士の加算をコンパイル時に検査できること
// Tはただのラベルとして扱いたいだけだが消費しないと怒られるので、std::marker::PhantdomDataを用いる
// 参考: https://keens.github.io/blog/2018/12/15/rustdetsuyomenikatawotsukerupart_1__new_type_pattern/
#[derive(Clone, Debug)]
pub struct Money<T> {
    amount: Decimal,
    currency: PhantomData<T>,
}

impl<T> Money<T> {
    fn new(amount: Decimal) -> Self {
        Self {
            amount,
            currency: PhantomData::<T>,
        }
    }
}

impl<T> Add for Money<T> {
    type Output = Money<T>;

    fn add(self, other: Money<T>) -> Self::Output {
        Self::new(self.amount + other.amount)
    }
}

pub enum JPY {}

pub enum USD {}

#[test]
fn test_phantom_money() {
    let jpy_1 = Money::<JPY>::new(Decimal::new(1, 0));
    let jpy_2 = Money::<JPY>::new(Decimal::new(2, 0));

    let usd = Money::<USD>::new(Decimal::new(3, 0));

    let result = jpy_1 + jpy_2; // コンパイルOk
                                // let invalid_result = jpy_1 + usd; //コンパイルエラー
}
