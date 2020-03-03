# 概要
ddd-in-rustは、DDDの実装パターンをRustで表現すること試みたリポジトリです。

DDDの実装パターンは、「[ドメイン駆動設計入門 ボトムアップでわかる！ドメイン駆動設計の基本](https://www.amazon.co.jp/dp/B082WXZVPC/ref=dp-kindle-redirect?_encoding=UTF8&btkr=1)」(著: 成瀬 允宣氏)という書籍のサンプルコードをベースにRustで書いています。（一部、書籍には出ていないパターンも試しています）

# 実装内容
`/src`配下にソースコードを置いています。
ディレクトリは「ドメイン駆動設計入門」のチャプター単位でまとめています。

DDDにおける以下の概念をRustで表現しています。
## Done
* Entity
* Value Object
* Domain Service
* Application Service

## WIP
* Dependency Injection
  * Cake pattern
* Repository

## TODO
* Factory
* Aggregate
* Specification
