use worker::{Request, Response, Result};

use forms_models::question::{
    Question, QuestionCreate, QuestionDelete, QuestionJs, QuestionRead, QuestionUpdate,
};
use forms_models::shared::{D1EntityCreate, D1EntityDelete, D1EntityUpdate};
use forms_models::D1EntityQueries;
use forms_shared::db::D1Action;
use forms_shared::{get_body, FormsResponse, WorkerHttpResponse};

use crate::admins::needs_admin;
use crate::shared::{error_wrapper, needs_auth};
use crate::RouterContext;

pub async fn get_all(req: Request, ctx: RouterContext) -> Result<Response> {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        let form_id = ctx.param("form_id").unwrap().parse().unwrap();

        needs_auth(&mut req, &db, Some(form_id)).await?;

        let body = QuestionRead {
            id: None,
            form_id: Some(form_id),
        };

        let forms = Question::read_query(&db, body)
            .all_into::<QuestionJs, Question>()
            .await?;

        let res = serde_json::to_string(&forms).unwrap();

        FormsResponse::ok(res)
    })
    .await
}

pub async fn post(req: Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        let form_id = ctx.param("form_id").unwrap().parse().unwrap();

        needs_admin(&mut req, &db, &ctx, Some(form_id)).await?;

        let body = QuestionCreate::from_req(&mut req, &ctx).await?;

        D1EntityCreate::create_query(body, &db).run().await?;

        FormsResponse::ok("OK")
    })
    .await
}

pub async fn patch(req: Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        needs_admin(&mut req, &db, &ctx, None).await?;

        let body = get_body::<QuestionUpdate>(&mut req).await?;

        D1EntityUpdate::update_query(body, &db).run().await?;

        FormsResponse::ok("OK")
    })
    .await
}

pub async fn delete(req: Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        needs_admin(&mut req, &db, &ctx, None).await?;

        let body = QuestionDelete {
            id: ctx.param("id").unwrap().parse().unwrap(), // FIXME: Handle parse error
        };

        D1EntityDelete::delete_query(body, &db).run().await?;

        FormsResponse::ok("OK")
    })
    .await
}
