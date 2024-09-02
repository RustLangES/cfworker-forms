use forms_models::answer::{
    Answer, AnswerCreate, AnswerCreateReq, AnswerJs, AnswerRead, AnswerUpdate,
};
use forms_models::session::SessionUpdate;
use worker::{Request, Response, Result};

use forms_models::shared::{D1EntityCreate, D1EntityRead, D1EntityUpdate};
use forms_shared::db::D1Action;
use forms_shared::{get_body, FormsResponse, WorkerHttpResponse};

use crate::shared::{error_wrapper, needs_auth};
use crate::RouterContext;

pub async fn get_all(req: Request, ctx: RouterContext) -> Result<Response> {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        let form_id = ctx.param("form_id").unwrap().parse().unwrap();

        let session = needs_auth(&mut req, &db, Some(form_id)).await?;

        let body = AnswerRead {
            form_id,
            question_id: None,
            session_id: session.id,
        };

        let answer = D1EntityRead::read_query(body, &db)
            .all_into::<AnswerJs, Answer>()
            .await?;

        FormsResponse::json(
            200,
            &serde_json::json!({
                "errors": [],
                "success": true,
                "data": answer
            }),
        )
    })
    .await
}

pub async fn get(req: Request, ctx: RouterContext) -> Result<Response> {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        let form_id = ctx.param("form_id").unwrap().parse().unwrap();
        let question_id = ctx.param("id").unwrap().parse().unwrap();

        let session = needs_auth(&mut req, &db, Some(form_id)).await?;

        let body = AnswerRead {
            form_id,
            question_id: Some(question_id),
            session_id: session.id,
        };

        let Some(answer) = D1EntityRead::read_query(body, &db)
            .first_into::<AnswerJs, Answer>()
            .await?
        else {
            return FormsResponse::json(
                404,
                &serde_json::json!({
                    "errors": ["No answered yet"],
                    "success": false
                }),
            );
        };

        FormsResponse::json(
            200,
            &serde_json::json!({
                "errors": [],
                "success": true,
                "data": answer
            }),
        )
    })
    .await
}

pub async fn post(req: Request, ctx: RouterContext) -> WorkerHttpResponse {
    error_wrapper(req, ctx, |mut req, ctx, db| async move {
        let form_id = ctx.param("form_id").unwrap().parse().unwrap();
        let question_id = ctx.param("id").unwrap().parse().unwrap();

        let session = needs_auth(&mut req, &db, Some(form_id)).await?;

        let body = AnswerRead {
            form_id,
            question_id: Some(question_id),
            session_id: session.id,
        };

        let answer = D1EntityRead::read_query(body, &db)
            .first_into::<AnswerJs, Answer>()
            .await?;

        let steps;

        let answer_id = if let Some(answer) = answer {
            let AnswerCreateReq { data } = get_body(&mut req).await?;

            let body = AnswerUpdate {
                id: answer.id,
                data,
            };

            D1EntityUpdate::update_query(body, &db).run().await?;

            let mut session_steps = session.steps.clone();

            if let Some((idx, _)) = session
                .steps
                .iter()
                .enumerate()
                .find(|(_, step)| **step == answer.id)
            {
                // If is already answered, cut vector
                session_steps.truncate(idx);
            } else {
                // If is new answer, just append it
                session_steps.push(answer.id);
            }

            steps = Some(session_steps);

            answer.id
        } else {
            let body = AnswerCreate::from_req(&mut req, &ctx, session.id).await?;

            let new_answer = D1EntityCreate::create_query(body, &db)
                .all::<AnswerJs>()
                .await?;

            let new_answer_id = new_answer.first().unwrap().id;

            // If is new answer, just append it
            let mut session_steps = session.steps.clone();
            session_steps.push(new_answer_id);
            steps = Some(session_steps);

            new_answer_id
        };

        D1EntityUpdate::update_query(
            SessionUpdate {
                id: session.id,
                last_answer: Some(answer_id),
                steps,
                ..Default::default()
            },
            &db,
        )
        .run()
        .await?;

        FormsResponse::ok("OK")
    })
    .await
}
