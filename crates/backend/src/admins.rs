use forms_models::session::SessionComplete;
use forms_shared::{FormsResponse, IntoResponse};

use crate::shared::needs_auth_complete;
use crate::RouterContext;

pub fn get_admins(ctx: &RouterContext) -> Result<Vec<String>, worker::Response> {
    Ok(ctx
        .env
        .secret("ADMINS")
        .map_err(IntoResponse::into_response)?
        .to_string()
        .split(";")
        .map(ToOwned::to_owned)
        .collect())
}

pub async fn needs_admin(
    req: &mut worker::Request,
    db: &worker::D1Database,
    ctx: &RouterContext,
    form_id: Option<usize>,
) -> Result<SessionComplete, worker::Response> {
    let session = needs_auth_complete(req, &db, form_id).await?;
    let is_admin = if let Some(external_email) = &session.external_email {
        get_admins(&ctx)?.contains(external_email)
    } else {
        false
    };

    if !is_admin {
        return Err(FormsResponse::json(
            403,
            &serde_json::json!({ "errors": [ "Forbidden" ], "success": false }),
        )?);
    }

    Ok(session)
}
