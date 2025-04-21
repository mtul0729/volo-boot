default:
    just --list
    
[group('dev')]
rpc-user:
    cargo run -p user --bin server -- --config=./rpc/user/config/dev.toml
    
[group('dev')]
rpc-order:
    cargo run -p order --bin server -- --config=./rpc/order/config/dev.toml

[group('dev')]
api:
    cargo run -p api --bin server -- --config=./api/config/dev.toml

docker:
    sudo docker compose up -d

nacos-console:
    firefox http://127.0.0.1:10848/
