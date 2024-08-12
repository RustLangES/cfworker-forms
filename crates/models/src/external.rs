use serde::{Deserialize, Serialize};

use crate::create_queries;

#[derive(Deserialize)]
pub struct ExternalJs {
    pub id: usize,

    pub external_id: String,
    pub kind: String,
    pub email: String,
    pub token: String,

    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct External {
    pub id: usize,

    pub external_id: String,
    pub kind: String,
    pub email: String,
    pub token: String,

    pub created_at: time::OffsetDateTime,
}

#[derive(Debug)]
pub struct ExternalRead {
    pub external_id: Option<String>,
    pub token: Option<String>,
}

#[derive(Deserialize)]
pub struct ExternalCreate {
    pub external_id: String,
    pub kind: String,
    pub email: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct ExternalUpdate {
    pub id: usize,

    pub email: Option<String>,

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
            email,
            kind,
            token,
            created_at,
        }: ExternalJs,
    ) -> Self {
        Self {
            id,
            external_id,
            email,
            kind,
            token,
            created_at: time::OffsetDateTime::from_unix_timestamp(created_at).unwrap(),
        }
    }
}

create_queries! {
    External where select_all = "id, external_id, kind, email, token, created_at",
    ExternalRead where select = with external; [
        external?.external_id;
        external?.token;
    ],
    ExternalCreate where create = with external; [
        external.external_id,
        external.kind,
        external.email,
        external.token,
    ],
    ExternalUpdate where update = with external; {
        where = [ external.id; ];
        set = [
            &external?.email;
            &external?.token;
        ];
    },
    ExternalDelete where delete = |external, db| {
        db
            .prepare("UPDATE External SET token = NULL WHERE token = ?")
            .bind(&[external.token.into()])
            .map_err(|err| format!("{err}"))
    },
}
