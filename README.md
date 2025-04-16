# volo-http 和 volo-rpc整合示例
## 目录结构
* /api: 网关模块
* /common: 放一些公共组件代码, 如日志配置等
* /rpc: 放rpc服务

## 相关issue
[issue](https://github.com/cloudwego/volo/issues/550)

## 功能进度
- [x] 基于volo的http网关代码  
- [x] 基于volo的grpc服务代码
- [x] 使用nacos作为服务注册发现中心
- [x] 从http网关到调用grpc服务的调用代码
- [x] http网关模块集成prometheus
- [ ] 限流组件
- [ ] 拦截请求header或参数
- [ ] 性能强悍(能抗住超高QPS)

## 说明
目前这个示例项目架构还没在生产环境验证，只是自己学习和测试用

交流学习群:
![img.png](feishu_chat.png)