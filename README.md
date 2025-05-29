# OTA_Sample
这是一个使用Rust作为后端，前端为html+css+js构建的OTA服务端，OTA客户端为ESP32；实现的简单嵌入式OTA服务

windows shell命令测试（linux可以把反斜杠去掉）

    curl -X POST -H "Content-Type: application/json" -d {\"message\":\"Hello\",\"timetamp\":\"12\"} http://127.0.0.1:8081/api/data //测试

# 注意事项
    windows下openssl环境可能有问题，参考：[rust引入含有openssl相关包报错（openssl未找到和编译运行报错等相关问题）解决方案
](https://www.cnblogs.com/jiajie6591/p/18631319)