# 概要
ddd-in-rustは、DDDの実装パターンをRustで表現すること試みたリポジトリです。

DDDの実装パターンは、「[ドメイン駆動設計入門 ボトムアップでわかる！ドメイン駆動設計の基本](https://www.amazon.co.jp/dp/B082WXZVPC/ref=dp-kindle-redirect?_encoding=UTF8&btkr=1)」(著: 成瀬 允宣氏)という書籍のサンプルコードをベースにRustで書いています。（一部、書籍には出ていないパターンも試しています）

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

# サンプルアプリケーション

## 起動
```shell
$ cargo run --bin sample
```
## サンプルリクエスト

### CreateUser
```shell
$ curl -X PUT -H 'Content-Type:application/json' -D - localhost:8080/users/ -d '{"name": "kuwana-kb", "mail_address": "kuwanakb@hoge.com"}'

HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 7
date: Sun, 08 Mar 2020 16:28:48 GMT

success
```

### GetUserByName
```shell
$ curl -X GET -H 'Content-Type:application/json' -D - localhost:8080/users/ -d '"kuwana-kb"'

HTTP/1.1 200 OK
content-type: application/json
content-length: 54
date: Sun, 08 Mar 2020 16:29:01 GMT

{"id":"01E2XFMGYRG35405W523R0PYYZ","name":"kuwana-kb"}
```
