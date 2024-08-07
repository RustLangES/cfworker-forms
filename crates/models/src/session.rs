use serde::{Deserialize, Serialize};

use crate::create_queries;

#[derive(Deserialize)]
pub struct SessionJs {
    pub id: usize,
    pub form_id: usize,

    pub external_id: Option<usize>,
    pub token: String,

    pub created_at: i64,
}

#[derive(Deserialize)]
pub struct SessionCompleteJs {
    pub id: usize,
    pub form_id: usize,
    pub created_at: i64,

    pub external_id: Option<usize>,
    pub external_kind: Option<String>,
    pub external_email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Session {
    pub id: usize,
    pub form_id: usize,
    pub external_id: Option<usize>,
    pub token: String,

    pub created_at: time::OffsetDateTime,
}

#[derive(Debug, Serialize)]
pub struct SessionComplete {
    pub id: usize,
    pub form_id: usize,
    pub created_at: time::OffsetDateTime,

    pub external_id: Option<usize>,
    pub external_kind: Option<String>,
    pub external_email: Option<String>,
}

#[derive(Debug)]
pub struct SessionRead {
    pub token: String,
    pub form_id: Option<usize>,
    pub complete: bool,
}

#[derive(Deserialize)]
pub struct SessionCreate {
    pub form_id: usize,
    pub external_id: Option<usize>,
    pub token: String,
}

#[derive(Deserialize)]
pub struct SessionUpdate {
    pub id: usize,
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
            form_id,
            external_id,
            token,
            created_at,
        }: SessionJs,
    ) -> Self {
        Self {
            id,
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
            form_id,
            created_at,

            external_id,
            external_email,
            external_kind,
        }: SessionCompleteJs,
    ) -> Self {
        Self {
            id,
            form_id,
            created_at: time::OffsetDateTime::from_unix_timestamp(created_at).unwrap(),
            external_id,
            external_kind,
            external_email,
        }
    }
}

create_queries! {
    Session where select_all = "id, form_id, external_id",
    SessionRead where select = |session, db| {
        let mut queries = vec![];
        let mut args = vec![];

        args.push(session.token.into());

        let query = if session.complete {
            queries.push("Session.token = ?");

            if let Some(form_id) = session.form_id {
                queries.push("Session.form_id = ?");
                args.push(form_id.into());
            }

            "SELECT \
                Session.id, Session.form_id, Session.external_id, Session.created_at, \
                External.kind AS external_kind, External.email AS external_email \
             FROM Session \
             LEFT JOIN External ON Session.external_id = External.id
             WHERE "
        } else {
            queries.push("token = ?");

            if let Some(form_id) = session.form_id {
                queries.push("form_id = ?");
                args.push(form_id.into());
            }

            "SELECT * FROM Session WHERE "
        }.to_owned();

        db
            .prepare(query + &queries.join(" AND "))
            .bind(&args)
            .map_err(|err| format!("{err}"))
    },
    SessionCreate where create = with session; [ session.form_id, session?.external_id, session.token, ],
    SessionUpdate where update = with session; {
        where = [ id = session.id ];
        set = [ session.external_id, session.token, ];
    },
    SessionDelete where delete = |session, db| {
        db
            .prepare("UPDATE Session SET token = NULL WHERE token = ?")
            .bind(&[session.token.into()])
            .map_err(|err| format!("{err}"))
    },
}
