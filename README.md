## OpenAPI Generator(Swagger)でrust-serverのコードを生成して、GET/POSTメソッドを呼び出すまで

という記事のサンプルコードです。

### 動作環境
手元のUbuntu20.04 on WSL2 で動作をしました。

### 構成
```
$ tree -L 3
.
├── Makefile
├── README.md
├── database
│   ├── Dockerfile
│   ├── Makefile
│   └── init
│       ├── ddl.sql
│       └── dml.sql
├── docker-compose.yml
└── server
    └── api
        ├── Cargo.lock
        ├── Cargo.toml
        ├── Makefile
        ├── README.md
        ├── api
        ├── docs
        ├── examples
        ├── openapi.yaml
        ├── openapitools.json
        ├── src
        └── target
```

### 機能概要

変化するグローバルIPを定期的にチェックし、最新のグローバルIPを取得できるように開発した機能をもとにしており、以下の機能があります。
- GETメソッドで最新のIPアドレスと最後に確認された日時を取得する
- POSTメソッドでIPアドレスを送信すると、最新のものと同じ場合は確認日時を更新し、異なる場合は新規登録する

### 使用方法

DBの立ち上げとAPIサーバーの起動
```
make run
```

メソッドの呼び出しは`make run`と別プロセスで行ってください。

GETメソッドの呼び出し
```
make curl-get
```

POSTメソッドの呼び出し
登録する内容を`IP_ADDRESS=x.x.x.x`として指定することができます。
デフォルトでは`IP_ADDRESS=1.1.1.1`となっています。
```
make curl-post IP_ADDRESS=10.10.10.10
```

終了時はプロセスを終了させ、以下コマンドでDBを停止させてください。
```
make down
```