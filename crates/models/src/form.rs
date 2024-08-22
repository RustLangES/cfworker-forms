use serde::{Deserialize, Serialize};

use crate::create_queries;
use crate::question::QuestionDetails;

#[derive(Deserialize)]
pub struct FormJs {
    pub id: usize,
    pub title: String,
    pub require_login: u8,
    pub deleted: u8,
    pub edition: String,
    pub multiple_times: u8,

    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct Form {
    pub id: usize,

    pub title: String,
    pub edition: String,
    pub multiple_times: bool,
    pub require_login: bool,

    #[serde(skip)]
    pub deleted: bool,
    #[serde(serialize_with = "crate::shared::date_ser")]
    pub created_at: time::OffsetDateTime,
}

#[derive(Debug)]
pub struct FormRead {
    pub id: usize,
}

#[derive(Debug, Serialize)]
pub struct FormDetails {
    pub id: usize,
    pub title: String,
    pub require_login: bool,
    pub deleted: bool,
    pub edition: String,
    pub multiple_times: bool,

    #[serde(serialize_with = "crate::shared::date_ser")]
    pub created_at: time::OffsetDateTime,
    pub questions: Vec<QuestionDetails>,
}

#[derive(Deserialize)]
pub struct FormCreate {
    pub title: String,
    pub require_login: bool,
    pub edition: String,
    pub multiple_times: bool,
}

#[derive(Deserialize)]
pub struct FormUpdate {
    pub id: usize,
    pub title: Option<String>,
    pub require_login: Option<bool>,
    pub edition: Option<String>,
    pub multiple_times: Option<bool>,
}

#[derive(Deserialize)]
pub struct FormDelete {
    pub id: usize,
}

impl Form {
    pub fn into_details(self, questions: Vec<QuestionDetails>) -> FormDetails {
        FormDetails {
            id: self.id,
            title: self.title,
            require_login: self.require_login,
            deleted: self.deleted,
            multiple_times: self.multiple_times,
            edition: self.edition,
            created_at: self.created_at,
            questions,
        }
    }
}

impl From<FormJs> for Form {
    fn from(
        FormJs {
            id,
            title,
            require_login,
            deleted,
            edition,
            created_at,
            multiple_times,
        }: FormJs,
    ) -> Self {
        Self {
            id,
            title,
            require_login: require_login == 1,
            deleted: deleted == 1,
            edition,
            multiple_times: multiple_times == 1,
            created_at: time::OffsetDateTime::from_unix_timestamp(created_at).unwrap(),
        }
    }
}

create_queries! {
    Form where select_all = [ id, title, require_login, deleted, created_at, edition, multiple_times ],
    FormRead where select = with form; [ form.id; ],
    FormCreate where create = with form; [ form.title, form.require_login, form.edition, form.multiple_times, ],
    FormUpdate where update = with form; {
        where = [ form.id; ];
        set = [
            &form?.title;
            form?.require_login;
            &form?.edition;
            form?.multiple_times;
        ];
    },
    FormDelete where delete = with form; [ form.id; ],
}
