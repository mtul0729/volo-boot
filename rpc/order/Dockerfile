# 第一阶段：构建
FROM rust:latest as builder

# 设置工作目录
WORKDIR /app

# 注意：构建时应该在项目根目录下运行 docker build -f rpc/order/Dockerfile .
# 这样才能正确包含所有依赖

# 复制整个项目
COPY . .

# 构建Order服务
RUN cargo build --release -p order

# 第二阶段：运行
FROM debian:bullseye-slim

WORKDIR /app

# 安装必要的系统依赖
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 从构建阶段拷贝编译好的二进制文件
COPY --from=builder /app/target/release/order-server /app/server

# 创建配置目录（运行时会挂载）
RUN mkdir -p /app

# 暴露Order服务端口
EXPOSE 50001

# 设置入口点，配置路径作为命令行参数传入
ENTRYPOINT ["/app/server"]
CMD ["--config=/app/config.toml"]