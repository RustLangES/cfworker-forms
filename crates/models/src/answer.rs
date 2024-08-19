use forms_shared::get_body;
use serde::{Deserialize, Serialize};
use worker::RouteContext;

use crate::create_queries;

#[derive(Deserialize)]
pub struct AnswerJs {
    pub id: usize,
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct Answer {
    pub id: usize,
    pub data: String,
}

#[derive(Debug)]
pub struct AnswerRead {
    pub form_id: usize,
    pub question_id: Option<usize>,
    pub session_id: usize,
}

#[derive(Deserialize)]
pub struct AnswerCreate {
    pub form_id: usize,
    pub question_id: usize,
    pub session_id: usize,

    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct AnswerUpdate {
    pub id: usize,

    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct AnswerDelete {
    pub form_id: usize,
    pub question_id: usize,
    pub session_id: usize,
}

impl TryFrom<AnswerJs> for Answer {
    type Error = serde_json::Error;

    fn try_from(AnswerJs { id, data }: AnswerJs) -> Result<Self, Self::Error> {
        Ok(Self { id, data })
    }
}

#[derive(Deserialize)]
pub struct AnswerCreateReq {
    pub data: String,
}

impl AnswerCreate {
    pub async fn from_req(
        req: &mut worker::Request,
        route: &RouteContext<()>,
        session_id: usize,
    ) -> Result<Self, worker::Response> {
        let AnswerCreateReq { data } = get_body(req).await?;
        let form_id: usize = route.param("form_id").unwrap().parse().unwrap();
        let question_id: usize = route.param("id").unwrap().parse().unwrap();

        Ok(Self {
            question_id,
            form_id,
            session_id,
            data,
        })
    }
}

create_queries! {
    Answer where select_all = "id, data",
    AnswerRead where select = with answer; [
        answer.form_id;
        answer?.question_id;
        answer.session_id;
    ],
    AnswerCreate where create = with answer; [
        answer.form_id,
        answer.question_id,
        answer.session_id,
        answer.data,
    ],
    AnswerUpdate where update = with answer; {
        where = [
            answer.id;
        ];
        set = [
            data = answer.data.clone();
        ];
    },
    AnswerDelete where delete = with answer; [
        answer.form_id;
        answer.question_id;
        answer.session_id;
    ],
}
