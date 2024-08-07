use serde::{Deserialize, Serialize};

use crate::create_queries;

#[derive(Deserialize)]
pub struct ExternalJs {
    pub id: usize,

    pub external_id: usize,
    pub external_kind: String,
    pub external_email: String,
    pub token: String,

    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct External {
    pub id: usize,

    pub external_id: usize,
    pub external_kind: String,
    pub external_email: String,
    pub token: String,

    pub created_at: time::OffsetDateTime,
}

#[derive(Debug)]
pub struct ExternalRead {
    pub token: String,
}

#[derive(Deserialize)]
pub struct ExternalCreate {
    pub form_id: usize,
    pub external_id: usize,
    pub external_kind: String,
    pub external_email: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct ExternalUpdate {
    pub id: usize,

    pub external_id: Option<usize>,
    pub external_kind: Option<String>,
    pub external_email: Option<String>,

    pub token: Option<String>,
}

#[derive(Deserialize)]
pub struct ExternalDelete {
    pub token: String,
}

impl From<ExternalJs> for External {
    fn from(
        ExternalJs {
            id,
            external_id,
            external_email,
            external_kind,
            token,
            created_at,
        }: ExternalJs,
    ) -> Self {
        Self {
            id,
            external_id,
            external_email,
            external_kind,
            token,
            created_at: time::OffsetDateTime::from_unix_timestamp(created_at).unwrap(),
        }
    }
}

create_queries! {
    External where select_all = "id, form_id, external_id",
    ExternalRead where select = with external; [ external.token, ],
    ExternalCreate where create = with external; [
        external.external_id,
        external.external_kind,
        external.external_email,
        external.token,
    ],
    ExternalUpdate where update = with external; {
        where = [ id = external.id ];
        set = [ external.external_id, external.token, ];
    },
    ExternalDelete where delete = |external, db| {
        db
            .prepare("UPDATE External SET token = NULL WHERE token = ?")
            .bind(&[external.token.into()])
            .map_err(|err| format!("{err}"))
    },
}
