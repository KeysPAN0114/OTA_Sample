# OTA_Sample
这是一个使用Rust作为后端，前端为html+css+js构建的OTA服务端，OTA客户端为ESP32；实现的简单嵌入式OTA服务

# 环境安装事项
本例程不提供安装教程；教程中使用环境如下：  
&emsp;&emsp;系统:ubuntu20.04  
&emsp;&emsp;Rust最新版本  
&emsp;&emsp;Nginx最新版本  
# NGINX配置
```Shell
    vim etc/nginx/nginx.conf    #修改nginx配置文件
```    
配置内容位于doc下的nginx.conf：[Nginx配置文件](/doc/nginx.conf)
配置完成nginx后需要重新启动
```Shell
    sudo systemctl restart nginx
```
# 运行
将Server目录复制到目标文件夹，执行 cargo run 
执行以下命令可以测试rust后台是否运行（注意，在终端运行cargo run时，会占用当前终端，可以转到后台运行或者打开新终端运行下面命令，不能关闭该终端）
windows shell命令测试（linux可以把反斜杠去掉）

    curl -X POST -H "Content-Type: application/json" -d {\"message\":\"Hello\",\"timetamp\":\"12\"} http://127.0.0.1:8081/api/data //测试

# 运行结果
&emsp;&emsp;上传文件后可以使用类似的链接访问文件：http://127.0.0.1/file/20250512_225046_Client.bin
# 注意事项
windows下openssl环境可能有问题，参考：[rust引入含有openssl相关包报错（openssl未找到和编译运行报错等相关问题）解决方案](https://www.cnblogs.com/jiajie6591/p/18631319)