# sync_code_server

## 项目介绍

一个基于rust的验证码同步服务，用于Android客户端的验证码同步到指定的设备上。服务端暴露了一个接口

- POST /api/message  客户端获取最新的验证码{"message":code} 存储到服务端的redis 中

- GET /api/message  客户端Chrome 扩展点获取最新的验证码加密数据并解密

## 项目结构

- `src/main.rs`：主文件，用于启动服务。
- `src/config.rs`：配置文件，用于存储配置信息。
- `src/handler.rs`：处理请求的文件，用于处理客户端的请求。
- `src/model.rs`：模型文件，用于存储模型信息。
- `src/utils.rs`：工具文件，用于存储工具函数。

## 项目依赖

- `tokio`：用于异步编程。
- `serde`：用于序列化和反序列化。
- `sqlite`：用于存储验证码信息。
- `redis`：用于sqlite的rust绑定。

## 项目运行

```bash
sudo apt install redis gcc g++ cmake
cargo run
``` 


## 项目配置

- `config.toml`：配置文件，用于存储配置信息。

## 项目启动

```bash
cargo run
```

## 项目测试

```bash
cargo test
```

## 项目打包

```bash
cargo build --release
```

## 项目部署

```bash

sudo cp sync_code /usr/local/bin/sync_code
```

## 项目启动

```bash
sudo vim /etc/systemd/system/sync_code.service

```bash
[Unit]
Description=Sync Code Service
After=network.target

[Service]
ExecStart=/usr/local/sync_code/sync_code
WorkingDirectory=/usr/local/sync_code
Restart=always
User=nobody
Group=nogroup
Environment="PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"

[Install]
WantedBy=multi-user.target

```


## 客户端
请在 https://github.com/monkey-wenjun/auto_sync_captcha/releases/tag/v1.0 下载客户端
