[TOC]
# watchdog 接口文档

## 一. 修订记录

| 版本 | 日期 | 修订人 | 说明 |
| --------------- | ---------- | ------ | ------- |
| v1.0 | 2019-08-01 | liujun | 创建 |



## 二. 说明

### 1. topic
- **前缀**: 所有的topic前面均为 /api



## 三. 接口
### 1. 停止进程
- **method**: DELETE
- **topic**: /stop
- **header**:
    | key | value |
    | name | 进程名称 |
- **request**: None
- **response**:
    - **format**: json
    - **body**:
    ```
    {
        "result": bool,
        "status": int,
        "message": string
    }
    ```

### 2. 重启进程
- **method**: PUT
- **topic**: /restart
- **header**:
    | key | value |
    | name | 进程名称 |
- **request**: None
- **response**:
    - **format**: json
    - **body**:
    ```
    {
        "result": bool,
        "status": int,
        "message": string
    }
    ```

### 3. 停止全部进程
- **method**: DELETE
- **topic**: /stop/all
- **header**: None
- **request**: None
- **response**:
    - **format**: json
    - **body**:
    ```
    {
        "result": bool,
        "status": int,
        "message": string
    }
    ```

### 4. 重启全部进程
- **method**: PUT
- **topic**: /restart/all
- **header**: None
- **request**: None
- **response**:
    - **format**: json
    - **body**:
    ```
    {
        "result": bool,
        "status": int,
        "message": string
    }
    ```

### 5. 获取全部配置
- **method**: GET
- **topic**: /config
- **header**: None
- **request**: None
- **response**:
    - **format**: json
    - **body**:
    ```
    {
        "data": [
            {
                "name": string,
                "execute": string,
                "args": [string],
                "directory": string,
                "isAuto": bool
            }
        ],
        "result": bool,
        "status": int,
        "message": string
    }
    ```
- **desc**:
    ```
    name: 区别所有进程的唯一指标
    execute: 可执行文件的名称, 该项可为空, 如果为空, 将取 args 的第一位作为执行命令
    args: 命令行参数, 如果 execute 参数为空, 这里的第一位必须是执行命令
    directory: 进程运行路径
    isAuto: 是否自动启动
    ```

### 6. 重新加载 (比对现有进程列表和输入进程列表)
- **method**: PUT
- **topic**: /reload
- **header**: None
- **request**:
    - **format**: json
    - **body**:
    ```
    {
        "processList": [
            {
                "name": string,
                "execute": string,
                "args": [string],
                "directory": string,
                "isAuto": bool
            }
        ]
    }
    ```
- **response**:
    - **format**: json
    - **body**:
    ```
    {
        "result": bool,
        "status": int,
        "message": string
    }
    ```
- **desc**:
    ```
    该接口不会更新配置文件, watchdog 服务重启后将不会生效
    ```

### 7. 更新配置文件后重新加载
- **method**: PUT
- **topic**: /save/before/reload
- **header**: None
- **request**:
    - **format**: json
    - **body**:
    ```
    {
        "processList": [
            {
                "name": string,
                "execute": string,
                "args": [string],
                "directory": string,
                "isAuto": bool
            }
        ]
    }
    ```
- **response**:
    - **format**: json
    - **body**:
    ```
    {
        "result": bool,
        "status": int,
        "message": string
    }
    ```
- **desc**:
    ```
    该接口会先更新配置文件, 再更新进程状态
    ```

### 8. 获取一个进程状态请求
- **method**: GET
- **topic**: /one/process/status
- **header**:
    | key | value |
    | name | 进程名称 |
- **request**: None
- **response**:
    - **format**: json
    - **body**:
    ```
    {
        "data": {
            "pid": int,
            "runTime": string,
            "status": string,
            "name": string
        },
        "result": bool,
        "status": int,
        "message": string
    }
    ```
- **desc**:
    ```
    ```

### 9. 获取全部进程状态请求
- **method**: GET
- **topic**: /all/process/status
- **header**: None
- **request**: None
- **response**:
    - **format**: json
    - **body**:
    ```
    {
        "data": [
            {
                "pid": int,
                "runTime": string,
                "status": string,
                "name": string
            }
        ],
        "result": bool,
        "status": int,
        "message": string
    }
    ```
- **desc**:
    ```
    ```
