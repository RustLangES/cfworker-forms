use forms_shared::get_body;
use serde::{Deserialize, Serialize};
use worker::wasm_bindgen::JsValue;
use worker::RouteContext;

use crate::create_queries;

#[derive(Deserialize)]
pub struct QuestionJs {
    pub id: usize,
    pub form_id: usize,

    pub title: String,
    pub description: String,

    pub r#type: String,
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct Question {
    pub id: usize,
    pub form_id: usize,

    pub title: String,
    pub description: String,

    pub r#type: String,
    pub data: serde_json::Value,
}

#[derive(Debug)]
pub struct QuestionRead {
    pub id: Option<usize>,
    pub form_id: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct QuestionDetails {
    pub id: usize,

    pub title: String,
    pub description: String,

    pub r#type: String,

    pub data: serde_json::Value,
}

#[derive(Deserialize)]
pub struct QuestionCreate {
    pub form_id: usize,

    pub title: String,
    pub description: String,

    pub r#type: String,

    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub data: JsValue,
}

#[derive(Debug, Deserialize)]
pub struct QuestionUpdate {
    pub id: usize,

    pub title: Option<String>,
    pub description: Option<String>,

    pub r#type: Option<String>,

    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub data: JsValue,
}

#[derive(Debug, Deserialize)]
pub struct QuestionDelete {
    pub id: usize,
}

impl Question {
    pub fn into_details(self) -> QuestionDetails {
        QuestionDetails {
            id: self.id,
            title: self.title,
            description: self.description,
            r#type: self.r#type,
            data: self.data,
        }
    }
}

impl TryFrom<QuestionJs> for Question {
    type Error = serde_json::Error;

    fn try_from(
        QuestionJs {
            id,
            form_id,
            title,
            description,
            r#type,
            data,
        }: QuestionJs,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            id,
            form_id,
            title,
            description,
            r#type,
            data: serde_json::from_str(&data)?,
        })
    }
}

#[derive(Deserialize)]
struct QuestionCreateReq {
    pub title: String,
    pub description: String,

    pub r#type: String,

    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub data: JsValue,
}

impl QuestionCreate {
    pub async fn from_req(
        req: &mut worker::Request,
        route: &RouteContext<()>,
    ) -> Result<Self, worker::Response> {
        let QuestionCreateReq {
            title,
            description,
            r#type,
            data,
        } = get_body(req).await?;
        let form_id: usize = route.param("form_id").unwrap().parse().unwrap();

        Ok(Self {
            form_id,
            title,
            description,
            r#type,
            data,
        })
    }
}

create_queries! {
    Question where select_all = "id, title, description, type, data, deleted",
    QuestionRead where select = with question; [ question?.id, question?.form_id, ],
    QuestionCreate where create = with question; [
        question.form_id,
        question.title,
        question.description,
        type = question.r#type,
        data = if !question.data.is_undefined() || !question.data.is_null() {
            Some(worker::js_sys::JSON::stringify(&question.data).map(String::from).unwrap())
        } else {
            None
        },
    ],
    QuestionUpdate where update = with question; {
        where = [ id = question.id ];
        set = [
            question.title,
            question.description,
            type = question.r#type,
            data = if !question.data.is_undefined() || !question.data.is_null() {
                Some(worker::js_sys::JSON::stringify(&question.data).map(String::from).unwrap())
            } else {
                None
            },
        ];
    },
    QuestionDelete where delete = with question; [ question.id, ],
}
