use std::future::Future;

use forms_models::session::{Session, SessionComplete, SessionCompleteJs, SessionJs, SessionRead};
use forms_models::shared::D1EntityRead;
use forms_shared::db::D1Action;
use forms_shared::{get_auth, FormsResponse, IntoResponse};

pub async fn error_wrapper<F>(
    req: worker::Request,
    ctx: worker::RouteContext<()>,
    handler: impl Fn(worker::Request, worker::RouteContext<()>, worker::D1Database) -> F,
) -> worker::Result<worker::Response>
where
    F: Future<Output = Result<worker::Response, worker::Response>>,
{
    let db = ctx.d1("DB").map_err(IntoResponse::into_response);

    let db = match db {
        Ok(o) => o,
        Err(why) => return Ok(why),
    };

    handler(req, ctx, db)
        .await
        .map_or_else(Result::Ok, Result::Ok)
}

pub async fn needs_auth(
    req: &mut worker::Request,
    db: &worker::D1Database,
    form_id: Option<usize>,
) -> Result<Session, worker::Response> {
    let Some(auth_token) = get_auth(req)? else {
        return Err(FormsResponse::json(
            403,
            &serde_json::json!({
                "errors": [ "Forbidden" ],
                "success": false
            }),
        )?);
    };

    let session = D1EntityRead::read_query(
        SessionRead {
            token: Some(auth_token),
            form_id,
            ..Default::default()
        },
        db,
    )
    .first_into::<SessionJs, Session>()
    .await?;

    let Some(session) = session else {
        return Err(FormsResponse::json(
            403,
            &serde_json::json!({
                "errors": [ "Session doesn't exist or expired" ],
                "success": false
            }),
        )?);
    };

    Ok(session)
}

pub async fn needs_auth_complete(
    req: &mut worker::Request,
    db: &worker::D1Database,
    form_id: Option<usize>,
) -> Result<SessionComplete, worker::Response> {
    let Some(auth_token) = get_auth(req)? else {
        return Err(FormsResponse::json(
            403,
            &serde_json::json!({
                "errors": [ "Forbidden" ],
                "success": false
            }),
        )?);
    };

    let session = D1EntityRead::read_query(
        SessionRead {
            token: Some(auth_token),
            form_id,
            complete: true,
            ..Default::default()
        },
        db,
    )
    .first_into::<SessionCompleteJs, SessionComplete>()
    .await?;

    let Some(session) = session else {
        return Err(FormsResponse::json(
            403,
            &serde_json::json!({
                "errors": [ "Session doesn't exist or expired" ],
                "success": false
            }),
        )?);
    };

    Ok(session)
}
