use std::future::Future;

use worker::D1Result;

use crate::{string_into_response, IntoResponse};

pub trait D1Action {
    fn run(self) -> impl Future<Output = Result<D1Result, worker::Response>>;
    fn try_run(self) -> impl Future<Output = Result<D1Result, worker::Error>>;

    fn first<T>(self) -> impl Future<Output = Result<Option<T>, worker::Response>>
    where
        T: for<'a> serde::Deserialize<'a>;

    fn first_into<T, B>(self) -> impl Future<Output = Result<Option<B>, worker::Response>>
    where
        T: for<'a> serde::Deserialize<'a>,
        B: TryFrom<T>,
        <B as TryFrom<T>>::Error: std::fmt::Display;

    fn all<T>(self) -> impl Future<Output = Result<Vec<T>, worker::Response>>
    where
        T: for<'a> serde::Deserialize<'a>;

    fn all_into<T, B>(self) -> impl Future<Output = Result<Vec<B>, worker::Response>>
    where
        T: for<'a> serde::Deserialize<'a>,
        B: TryFrom<T>,
        <B as TryFrom<T>>::Error: std::fmt::Display;
}

impl D1Action for Result<worker::D1PreparedStatement, String> {
    async fn run(self) -> Result<D1Result, worker::Response> {
        let res = self
            .map_err(string_into_response(500))?
            .run()
            .await
            .map_err(IntoResponse::into_response)?;

        res.error()
            .map_or(Result::Ok(()), Result::Err)
            .map_err(string_into_response(500))?;

        Ok(res)
    }

    async fn try_run(self) -> Result<D1Result, worker::Error> {
        let res = self.map_err(worker::Error::RustError)?.run().await?;

        res.error()
            .map_or(Result::Ok(()), Result::Err)
            .map_err(worker::Error::RustError)?;

        Ok(res)
    }

    async fn first<T>(self) -> Result<Option<T>, worker::Response>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        let res = self
            .map_err(string_into_response(500))?
            .first::<T>(None)
            .await
            .map_err(IntoResponse::into_response)?;

        Ok(res)
    }

    async fn first_into<T, B>(self) -> Result<Option<B>, worker::Response>
    where
        T: for<'a> serde::Deserialize<'a>,
        B: TryFrom<T>,
        <B as TryFrom<T>>::Error: std::fmt::Display,
    {
        let res = self
            .map_err(string_into_response(500))?
            .first::<T>(None)
            .await
            .map_err(IntoResponse::into_response)?
            .map(B::try_from)
            .map(|m| m.inspect_err(|err| worker::console_error!("Cannot parse: {err}")));

        Ok(res.and_then(|m| m.ok()))
    }

    async fn all<T>(self) -> Result<Vec<T>, worker::Response>
    where
        T: for<'a> serde::Deserialize<'a>,
    {
        let res = self
            .map_err(string_into_response(500))?
            .all()
            .await
            .map_err(IntoResponse::into_response)?;

        res.error()
            .map_or(Result::Ok(()), Result::Err)
            .map_err(string_into_response(500))?;

        res.results::<T>().map_err(IntoResponse::into_response)
    }

    async fn all_into<T, B>(self) -> Result<Vec<B>, worker::Response>
    where
        T: for<'a> serde::Deserialize<'a>,
        B: TryFrom<T>,
        <B as TryFrom<T>>::Error: std::fmt::Display,
    {
        Ok(self
            .all::<T>()
            .await?
            .into_iter()
            .map(B::try_from)
            .map(|m| m.inspect_err(|err| worker::console_error!("Cannot parse Question: {err}")))
            .filter_map(std::result::Result::ok)
            .collect())
    }
}
