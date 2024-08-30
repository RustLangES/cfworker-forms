use serde::{Deserialize, Serialize};

use crate::create_queries;

#[derive(Deserialize, Debug)]
pub struct SessionJs {
    pub id: usize,
    pub device_id: String,
    pub form_id: usize,

    pub external_id: Option<usize>,
    pub token: Option<String>,

    pub last_answer: Option<usize>,

    pub created_at: i64,
}

#[derive(Deserialize)]
pub struct SessionCompleteJs {
    pub id: usize,
    pub form_id: usize,
    pub device_id: String,
    pub created_at: i64,
    pub last_answer: Option<usize>,

    pub external_id: Option<usize>,
    pub external_kind: Option<String>,
    pub external_email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Session {
    pub id: usize,
    pub form_id: usize,
    pub device_id: String,
    pub external_id: Option<usize>,
    pub token: Option<String>,
    pub last_answer: Option<usize>,

    pub created_at: time::OffsetDateTime,
}

#[derive(Debug, Serialize)]
pub struct SessionComplete {
    pub id: usize,
    pub form_id: usize,
    pub created_at: time::OffsetDateTime,
    pub last_answer: Option<usize>,

    pub external_id: Option<usize>,
    pub external_kind: Option<String>,
    pub external_email: Option<String>,
}

#[derive(Debug, Default)]
pub struct SessionRead {
    pub token: Option<String>,
    pub device_id: Option<String>,
    pub external_id: Option<usize>,
    pub external_token: Option<String>,
    pub deleted: Option<bool>,
    pub form_id: Option<usize>,
    pub complete: bool,
}

#[derive(Deserialize)]
pub struct SessionCreate {
    pub form_id: usize,
    pub device_id: String,
    pub external_id: Option<usize>,
    pub token: String,
}

#[derive(Default, Deserialize)]
pub struct SessionUpdate {
    pub id: usize,
    pub last_answer: Option<usize>,
    pub external_id: Option<usize>,
    pub token: Option<String>,
}

#[derive(Deserialize)]
pub struct SessionDelete {
    pub token: String,
}

impl From<SessionJs> for Session {
    fn from(
        SessionJs {
            id,
            device_id,
            last_answer,
            form_id,
            external_id,
            token,
            created_at,
        }: SessionJs,
    ) -> Self {
        Self {
            id,
            device_id,
            last_answer,
            form_id,
            external_id,
            token,
            created_at: time::OffsetDateTime::from_unix_timestamp(created_at).unwrap(),
        }
    }
}

impl From<SessionCompleteJs> for SessionComplete {
    fn from(
        SessionCompleteJs {
            id,
            device_id: _,
            form_id,
            created_at,
            last_answer,

            external_id,
            external_email,
            external_kind,
        }: SessionCompleteJs,
    ) -> Self {
        Self {
            id,
            form_id,
            created_at: time::OffsetDateTime::from_unix_timestamp(created_at).unwrap(),
            last_answer,
            external_id,
            external_kind,
            external_email,
        }
    }
}

create_queries! {
    Session where select_all = [ id, form_id, external_id, device_id, last_answer, ],
    SessionRead where select = |session, db| {
        let mut queries = vec!["deleted = ?"];
        let mut args = vec![false.into()];

        let query = if session.complete {
            if let Some(token) = session.token {
                queries.push("Session.token = ?");
                args.push(token.into());
            }

            match (session.external_id, session.external_token, session.device_id) {
                (Some(external_id), Some(external_token), Some(device_id)) => {
                    queries.push("(Session.external_id = ? OR External.token = ? OR Session.device_id = ?)");
                    args.push(external_id.into());
                    args.push(external_token.into());
                    args.push(device_id.into());
                },
                (None, Some(external_token), Some(device_id)) => {
                    queries.push("(Session.device_id = ? OR External.token ?)");
                    args.push(device_id.into());
                    args.push(external_token.into());
                },
                (Some(external_id), None, None) => {
                    queries.push("Session.external_id = ?");
                    args.push(external_id.into());
                },
                (None, None, Some(device_id)) => {
                    queries.push("Session.device_id = ?");
                    args.push(device_id.into());
                },
                (_, _, _) => {}
            }

            if let Some(form_id) = session.form_id {
                queries.push("Session.form_id = ?");
                args.push(form_id.into());
            }

            if Some(true) == session.deleted {
                queries.push("Session.token = NULL");
            }

            "SELECT \
                Session.id, Session.form_id, Session.external_id, Session.created_at, \
                External.kind AS external_kind, External.email AS external_email \
             FROM Session \
             LEFT JOIN External ON Session.external_id = External.id
             WHERE "
        } else {
            if let Some(token) = session.token {
                queries.push("token = ?");
                args.push(token.into());
            }

            match (session.external_id, session.device_id) {
                (Some(external_id), Some(device_id)) => {
                    queries.push("(external_id = ? OR device_id = ?)");
                    args.push(external_id.into());
                    args.push(device_id.into());
                },
                (Some(external_id), None) => {
                    queries.push("external_id = ?");
                    args.push(external_id.into());
                },
                (None, Some(device_id)) => {
                    queries.push("device_id = ?");
                    args.push(device_id.into());
                },
                (None, None) => {}
            }

            if let Some(form_id) = session.form_id {
                queries.push("form_id = ?");
                args.push(form_id.into());
            }

            if Some(true) == session.deleted {
                queries.push("token ISNULL");
            }

            "SELECT * FROM Session WHERE "
        }.to_owned();

        worker::console_log!("SessionRead: {}", query.clone() + &queries.join(" AND "));
        worker::console_log!("SessionRead: {args:?}");
        db
            .prepare(query + &queries.join(" AND "))
            .bind(&args)
            .map_err(|err| format!("{err}"))
    },
    SessionCreate where create = with session; [ session.form_id, session.device_id, session?.external_id, session.token, ],
    SessionUpdate where update = with session; {
        where = [ session.id; ];
        set = [ session.external_id; session.token; session.last_answer; ];
    },
    SessionDelete where delete = |session, db| {
        db
            .prepare("UPDATE Session SET token = NULL WHERE token = ?")
            .bind(&[session.token.into()])
            .map_err(|err| format!("{err}"))
    },
}
