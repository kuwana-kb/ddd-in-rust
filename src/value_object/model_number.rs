/// 製品番号
// 製品番号がどのように構成されているかをコードとしてあらわす
struct ModelNumber {
    product_code: String,
    branch: String,
    lot: String,
}

impl ModelNumber {
    fn new(product_code: &str, branch: &str, lot: &str) -> Self {
        Self {
            product_code,
            branch,
            lot,
        }
    }
}

// Display trait を実装することで、文字列として出力する際の挙動を制御できる
// 以下の場合は、「{product_code}-{branch}-{lot}」で出力する
impl fmt::Display for ModelNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.product_code, self.branch, self.lot)
    }
}
