# 显示当前这条帮助信息
help:
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

# 启动docker容器,服务不使用docker启动,需要在本地直接运行服务

[group('dev')]
test: docker-up
    just rpc-user > ./tmp/rpc-user.log 2>&1 &
    just rpc-order > ./tmp/rpc-order.log 2>&1 &
    sleep 2
    just api > ./tmp/api.log 2>&1 &
    sleep 2
    ./tests/http_test.fish
    echo "日志文件为rpc-user.log,rpc-order.log,api.log"

[group('dev')]  
docker-up:
    sudo docker compose -f docker-compose-dev.yml up -d

# 构建所有服务的docker镜像,构建时间长,不建议在开发环境运行
[group('test')]
docker-build:
    cargo clean
    sudo COMPOSE_BAKE=true docker compose -f docker-compose-test.yml build