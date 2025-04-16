use volo::layer::LayerExt;
use crate::ServiceContext;
use user::user::{GetUserRequest, User};
use volo::loadbalance::RequestHash;
use volo::METAINFO;
use volo_http::request::ServerRequest;
use volo_http::{http::StatusCode, json::Json, server::extract::Query, Extension};

/// 通过id获取用户实体
pub async fn get_user(
    Extension(ctx): Extension<ServiceContext>,
    Query(param): Query<serde_json::Value>,
    _req: ServerRequest,
) -> Result<Json<User>, StatusCode> {
    let Some(rpc_cli) = ctx.rpc_cli_user else {
        return Err(StatusCode::GONE);
    };

    let Some(id) = param.get("id") else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let Some(str_id) = id.as_str() else {
        return Err(StatusCode::BAD_REQUEST);
    };


    let id: i64 = str_id.parse().map_err(|_| StatusCode::BAD_REQUEST)?;

    // 如果 load_balance 用的ConsistentHashBalance, 则需要在本地变量（类似于java中的ThreadLocal变量）设置RequestHash
    // 每个请求会自动创建本地变量 METAINFO, 然后在自己的方法里面直接用就行, 参考: https://docs.rs/tokio/latest/tokio/task/struct.LocalKey.html
    METAINFO.with(|m| {
        m.borrow_mut().insert(RequestHash(id as u64));
    });

    // 请求user rpc服务，然后返回
    let ret_user = rpc_cli
        .get_user(GetUserRequest {
            id: Some(id),
            username: Some("some name123".into()),
        })
        .await;
    match ret_user {
        Ok(u) => Ok(Json(u.into_inner())),
        Err(e) => {
            tracing::error!("get_user error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
