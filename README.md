# rust-actix-rest-api-boilerplate
A Rust RESTful API server with actix-web

## 安装
- 安装[Rust](https://www.rust-lang.org/)
- 安装[Docker](https://www.docker.com/) (可选)
- 安装[Docker Compose](https://github.com/docker/compose/releases) (可选)


首先安装[cargo-make](https://github.com/sagiegurari/cargo-make)，可以用cargo make命令启动一些脚本

```
$ cargo install --no-default-features --force cargo-make
```

```
$ git clone 本库地址
```

## 启动开发环境
```
$ cargo make dev
```

默认启用8080端口，可通过 http://localhost:8080/hello 测试是否启动成功


## 清理
```
$ cargo make clean
```

## 打包
```
$ cargo make build
```

## mac交叉编译linux
```
$ cargo make buildlinux
```

## 启动本地开发环境数据库(Docker,可选)
````
$ docker-compose up
````

通过docker-compose启动postgres和redis，postgres端口为5432, redis端口为6379。数据库可通过本地客户端工具连接进行操作和调试。

## 停止本地开发环境数据库(Docker, 可选)
````
$ docker-compose down
````

## 说明

各模块的解决方案和库的选择以简洁为主，尽量不选择过于复杂的方式，尽量让脚手架轻量。

### 关于web框架actix-web

[actix-web](https://actix.rs/)是rust下快速的异步web框架。底层异步库使用的[Tokio](https://tokio.rs/)，开发时注意使用异步的方式开发。

数据库连接池，redis连接池和配置文件的相关实例，在actix web启动的时候通过`app_data`传入actix。

```
App::new()
    .app_data(web::Data::new(AppState {
        config: settings.clone(),
        log: logger.clone(),
        db: db_pool.clone(),
        redis: redis_pool.clone(),
    }))
...
```

### 配置

配置库选择[config](https://github.com/mehcode/config-rs)，配置文件为 `data/config/app.toml`，配置数据可通过`web::Data`获取：

```
pub async fn hello(state: web::Data<AppState>) -> Result<web::HttpResponse, error::Error> {
    let name = state.config.get::<String>("app.name").unwrap();
    ...
}
```

### 日志

日志库选择[slog](https://github.com/slog-rs/slog)，支持异步，配置了日志文件和屏幕双输出。日志文件为 `data/logs/app.log`，actix中可通过如下方式记录日志：

```
pub async fn hello(state: web::Data<AppState>) -> Result<web::HttpResponse, error::Error> {
    ...
    info!(state.log, "hello {}", name);
    ...
}
```

### 数据库

数据库操作库选择的[sqlx](https://github.com/launchbadge/sqlx)，本例做了postgres的配置，可支持其他各种常用数据库。可通过`web::Data`获取数据库连接池。注意要使用异步方式开发。

### Redis

Redis操作库选择的[redis](https://github.com/mitsuhiko/redis-rs)，支持异步方式，使用[mobc](https://github.com/importcjj/mobc)配置的连接池。可通过`web::Data`获取redis连接池。注意要使用异步方式开发。

```
use redis::AsyncCommands;
...

pub async fn hello(state: web::Data<AppState>) -> Result<web::HttpResponse, error::Error> {
    let mut con = state.redis.get().await.unwrap();
    let val = con.get("my_key").await.unwrap();
    ...
}

```

### 错误格式

采用JSON数据结构。

```
{"errcode":100203,"errmsg":"Captcha not found"}
```
(其他语言的后端接口我们也是统一返回类似的错误码，这样对于前端不用关心后端使用什么语言，使用统一的错误格式即可)

actix中可使用`src/lib/error`中定义的错误类型输出错误：

```
use crate::lib::error;
...

return Err(error::new(400001, "姓名不能为空", 422));
...

```

注：暂时使用422的错误码输出错误，在当前的actix版本，400错误会被默认自定义错误输出覆盖，待后续解决。

### 输入验证

暂时没采用第三方库做输入验证，采用简单方式处理（沿用我其他语言脚手架的惯用方式），一些输入格式可通过 `src/lib/validator.rs` 中的正则判断，可在这里增加其他需要的验证。在actix中，通过?来验证和传递错误。

```
validator::uuid(uuid_var, "uuid")?;
validator::not_none(absent_number, "数值")?;
...
```

如果需要对输入的字符串判断非None并将Option自动转换为String，可采用`validator::required_str`函数：

```
let name = validator::required_str(name_param, "名称")?;
```
