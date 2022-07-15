# 离线签名

## 获取签名

-   req

    ```http
    GET /sign/{chain_id}/{id}/{tokenid}
    ```
    - chain_id:链id
    - id:订单id
    - tokenid:nft tokenid
-   res

    ```json
    {
    "id": 1,
    "tokenid": 2,
    "v": 28,
    "r": "0x00a9b33e2c227e3e8f8992cd5debc98c2ce2fc7b905173522f72c152a0f6a71c",
    "s": "0x4ec8fc9b201893ac35b3203f28acb8df60623f8887773ca6d455768683da445a"
    }
    ```
