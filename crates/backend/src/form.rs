use forms_models::session::SessionJs;
use worker::{Request, Response, Result};

use forms_models::{
    form::{Form, FormCreate, FormDelete, FormJs, FormRead, FormUpdate},
    question::{Question, QuestionJs},
    shared::{D1EntityCreate, D1EntityDelete, D1EntityRead, D1EntityUpdate},
    D1EntityQueries,
};
use forms_shared::db::D1Action;
use forms_shared::{get_auth, get_body, FormsResponse, WorkerHttpResponse};

use crate::admins::needs_admin;
use crate::session::get_device_id;
use crate::shared::error_wrapper;
use crate::RouterContext;

pub async fn get_all(req: Request, ctx: RouterContext) -> Result<Response> {
    error_wrapper(req, ctx, |_, _, db| async move {
        let forms = Form::read_all_query(&db).all_into::<FormJs, Form>().await?;
        FormsResponse::json(200, &forms)
    })
    .await
}

pub async fn get(req: Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        let form_id = ctx.param("id").unwrap().parse().unwrap();
        let body = FormRead { id: form_id };

        let Some(form) = D1EntityRead::read_query(body, &db)
            .first_into::<FormJs, Form>()
            .await?
        else {
            return FormsResponse::text(404, "Not Found");
        };

        let body = forms_models::question::QuestionRead {
            id: None,
            form_id: Some(form_id),
        };

        let questions = D1EntityRead::read_query(body, &db)
            .all_into::<QuestionJs, Question>()
            .await?;

        let body = if let Some(token) = get_auth(&mut req)? {
            Some(forms_models::session::SessionRead {
                external_token: Some(token),
                complete: true,
                ..Default::default()
            })
        } else if let Some(device_id) = get_device_id(&req) {
            Some(forms_models::session::SessionRead {
                device_id: Some(device_id),
                form_id: Some(form_id),
                ..Default::default()
            })
        } else {
            None
        };

        let last_answer = if let Some(body) = body {
            D1EntityRead::read_query(body, &db)
                .first::<SessionJs>()
                .await?
                .map(|session| session.last_answer)
                .flatten()
        } else {
            None
        };

        let questions = questions.into_iter().map(Question::into_details).collect();
        let form = form.into_details(questions, last_answer);

        FormsResponse::json(
            200,
            &serde_json::json!({
                "errors": [],
                "success": true,
                "data": form
            }),
        )
    })
    .await
}

pub async fn post(req: Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        needs_admin(&mut req, &db, &ctx, None).await?;

        let body = get_body::<FormCreate>(&mut req).await?;

        D1EntityCreate::create_query(body, &db).run().await?;

        FormsResponse::ok("OK")
    })
    .await
}

pub async fn patch(req: Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        needs_admin(&mut req, &db, &ctx, None).await?;

        let body = get_body::<FormUpdate>(&mut req).await?;

        D1EntityUpdate::update_query(body, &db).run().await?;

        FormsResponse::ok("OK")
    })
    .await
}

pub async fn delete(req: Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        needs_admin(&mut req, &db, &ctx, None).await?;

        let body = FormDelete {
            id: ctx.param("id").unwrap().parse().unwrap(), // FIXME: Handle parse error
        };

        D1EntityDelete::delete_query(body, &db).run().await?;

        FormsResponse::ok("OK")
    })
    .await
}
