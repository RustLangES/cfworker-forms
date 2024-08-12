use serde::Serializer;
use worker::{D1Database, D1PreparedStatement};

pub trait D1EntityQueries<Create, ReadAll, Read, Update, Delete>
where
    Create: D1EntityCreate,
    ReadAll: D1EntityReadAll,
    Read: D1EntityRead,
    Update: D1EntityUpdate,
    Delete: D1EntityDelete,
{
    fn create_query(
        d1: &D1Database,
        data: Create,
    ) -> std::result::Result<D1PreparedStatement, String> {
        data.create_query(d1)
    }

    fn read_all_query(d1: &D1Database) -> std::result::Result<D1PreparedStatement, String> {
        ReadAll::read_all_query(d1)
    }

    fn read_query(d1: &D1Database, data: Read) -> std::result::Result<D1PreparedStatement, String> {
        data.read_query(d1)
    }

    fn update_query(
        d1: &D1Database,
        data: Update,
    ) -> std::result::Result<D1PreparedStatement, String> {
        data.update_query(d1)
    }

    fn delete_query(
        d1: &D1Database,
        data: Delete,
    ) -> std::result::Result<D1PreparedStatement, String> {
        data.delete_query(d1)
    }
}

pub trait D1EntityReadAll {
    fn read_all_query(d1: &D1Database) -> std::result::Result<D1PreparedStatement, String>;
}

pub trait D1EntityRead {
    fn read_query(self, d1: &D1Database) -> std::result::Result<D1PreparedStatement, String>;
}

pub trait D1EntityCreate {
    fn create_query(self, d1: &D1Database) -> std::result::Result<D1PreparedStatement, String>;
}

pub trait D1EntityUpdate {
    fn update_query(self, d1: &D1Database) -> std::result::Result<D1PreparedStatement, String>;
}

pub trait D1EntityDelete {
    fn delete_query(self, d1: &D1Database) -> std::result::Result<D1PreparedStatement, String>;
}

pub fn date_ser<S>(val: &time::OffsetDateTime, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let v = val.date().to_string();

    ser.serialize_str(&v)
}
