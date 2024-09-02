use std::ops::Not;

use serde::{Deserialize, Serialize};

use crate::{create_queries, new_query};

#[derive(Deserialize, Debug)]
pub struct SessionJs {
    pub id: usize,
    pub device_id: String,
    pub form_id: usize,

    pub external_id: Option<usize>,
    pub token: Option<String>,

    pub last_answer: Option<usize>,
    pub steps: String,

    pub created_at: i64,
}

#[derive(Deserialize)]
pub struct SessionCompleteJs {
    pub id: usize,
    pub form_id: usize,
    pub device_id: String,
    pub created_at: i64,
    pub last_answer: Option<usize>,
    pub steps: String,

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
    pub steps: Vec<usize>,

    pub created_at: time::OffsetDateTime,
}

#[derive(Debug, Serialize)]
pub struct SessionComplete {
    pub id: usize,
    pub form_id: usize,
    pub created_at: time::OffsetDateTime,
    pub last_answer: Option<usize>,
    pub steps: Vec<usize>,

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

#[derive(Default, Deserialize, Debug)]
pub struct SessionUpdate {
    pub id: usize,
    pub last_answer: Option<usize>,
    pub external_id: Option<usize>,
    pub token: Option<String>,
    pub steps: Option<Vec<usize>>,
}

#[derive(Deserialize)]
pub struct SessionDelete {
    pub token: String,
}

fn parse_steps(steps: String) -> Vec<usize> {
    steps
        .split_terminator(";")
        .map(|step| step.parse::<usize>().unwrap())
        .collect()
}

fn serialize_steps(steps: Vec<usize>) -> String {
    steps
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(";")
}

impl From<SessionJs> for Session {
    fn from(
        SessionJs {
            id,
            device_id,
            last_answer,
            form_id,
            external_id,
            steps,
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
            steps: parse_steps(steps),
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
            steps,

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
            steps: parse_steps(steps),
            external_id,
            external_kind,
            external_email,
        }
    }
}

create_queries! {
    Session where select_all = [ id, form_id, external_id, device_id, last_answer, steps ],
    SessionRead where select = |session, db| {
        let mut queries = vec!["deleted = ?"];
        let mut args = vec![false.into()];
        let mut or_queries = None;
        let mut or_args = vec![];

        let query = if session.complete {
            if let Some(token) = session.token {
                queries.push("Session.token = ?");
                args.push(token.into());
            }

            let (or_query, or_args_) = new_query!(!;
                "Session.external_id" ?= session.external_id;
                "External.token" ?= session.external_token;
                "Session.device_id" ?= session.device_id;
            );

            if !or_query.is_empty() {
                or_queries = Some(format!("({})", or_query.join(" OR ")));
                or_args = or_args_;
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

            let (or_query, or_args_) = new_query!(!;
                "external_id" ?= session.external_id;
                // "External.token" ?= session.external_token;
                "device_id" ?= session.device_id;
            );

            if !or_query.is_empty() {
                or_queries = Some(format!("({})", or_query.join(" OR ")));
                or_args = or_args_;
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

        let queries = queries.join(" AND ");
        let or_queries = or_queries
            .take()
            .map(|s| s + queries
                .is_empty()
                .not()
                .then_some(" AND ")
                .unwrap_or_default())
            .unwrap_or(String::new());
        let query = query + &or_queries + &queries;

        or_args.append(&mut args);
        let args = or_args;

        worker::console_log!("SessionRead: {query}");
        worker::console_log!("SessionRead: {args:?}");

        db
            .prepare(query)
            .bind(&args)
            .map_err(|err| format!("{err}"))
    },
    SessionCreate where create = with session; [
        session.device_id,
        session?.external_id,
        session.form_id,
        steps = "",
        session.token,
    ],
    SessionUpdate where update = with session; {
        where = [ session.id; ];
        set = [
            session?.external_id;
            session?.last_answer;
            steps ?= session.steps.map(serialize_steps);
            session?.token;
        ];
    },
    SessionDelete where delete = |session, db| {
        db
            .prepare("UPDATE Session SET token = NULL WHERE token = ?")
            .bind(&[session.token.into()])
            .map_err(|err| format!("{err}"))
    },
}
