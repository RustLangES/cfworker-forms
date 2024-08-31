#[macro_export]
macro_rules! create_queries {
    (
        $(table = $table:literal;)?
        $ty:ty where select_all = $($select_all:tt)*
    ) => {
        $crate::macros::create_queries!{!queries ReadAll ($ty, $crate::macros::create_queries!(!table $ty, $($table)?)); $($select_all)*}
    };

    (!table $ty:ty,) => (stringify!($ty));
    (!table $ty:ty, $table:literal) => (stringify!($table));

    (!queries ReadAll ($ty:ty, $table:expr); | $db:ident | $body:block, $($tail:tt)*) => {
        $crate::macros::create_query!{!impl D1EntityReadAll for $ty : $table; | $db | $body }
        $crate::macros::create_queries!{!queries Read ($ty, $table, $ty); $($tail)*}
    };

    (!queries ReadAll ($ty:ty, $table:expr); [ $($select_all:tt)* ], $($tail:tt)*) => {
        $crate::macros::create_query!{!impl D1EntityReadAll for $ty : $table; [ $($select_all)* ] }
        $crate::macros::create_queries!{!queries Read ($ty, $table, $ty); $($tail)*}
    };

    (!queries Read ($cty:ty, $table:expr, $READ_ALL:ty); $ty:ty where select = | $self:ident, $db:ident | $body:block, $($tail:tt)*) => {
        $crate::macros::create_query!{!impl D1EntityRead for $ty : $table; | $self, $db | $body}
        $crate::macros::create_queries!{!queries Create ($cty, $table, $READ_ALL, $ty); $($tail)*}
    };

    (!queries Read ($cty:ty, $table:expr, $READ_ALL:ty); $ty:ty where select = with $self:ident; [$($set:tt)+], $($tail:tt)*) => {
        $crate::macros::create_query!{!impl D1EntityRead for $ty : $table; with $self; [$($set)+]}
        $crate::macros::create_queries!{!queries Create ($cty, $table, $READ_ALL, $ty); $($tail)*}
    };

    (!queries Create ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty); $ty:ty where create = | $self:ident, $db:ident | $body:block, $($tail:tt)*) => {
        $crate::macros::create_query!{!impl D1EntityCreate for $ty : $table; | $self, $db | $body}
        $crate::macros::create_queries!{!queries Update ($cty, $table, $READ_ALL, $READ, $ty); $($tail)*}
    };

    (!queries Create ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty);
        $ty:ty where create = with $self:ident; [ $($set:tt)+ ], $($tail:tt)*) => {

        $crate::macros::create_query!{!impl D1EntityCreate for $ty : $table; with $self; [$($set)+]}
        $crate::macros::create_queries!{!queries Update ($cty, $table, $READ_ALL, $READ, $ty); $($tail)*}
    };

    (!queries Update ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty, $CREATE:ty);
        $ty:ty where update = | $db:ident | $body:block, $($tail:tt)*) => {

        $crate::macros::create_query!{!impl D1EntityUpdate for $ty : $table; | $self, $db | $body}
        $crate::macros::create_queries!{!queries Delete ($cty, $table, $READ_ALL, $READ, $CREATE, $ty); $($tail)*}
    };

    (!queries Update ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty, $CREATE:ty);
        $ty:ty where update = with $self:ident; {
            where = [ $($where:tt)+ ];
            set = [ $($set:tt)+ ];
        }, $($tail:tt)*) => {

        $crate::macros::create_query!{!impl D1EntityUpdate for $ty : $table;
            with $self; { where = [$($where)+]; set = [$($set)+]; }}
        $crate::macros::create_queries!{!queries Delete ($cty, $table, $READ_ALL, $READ, $CREATE, $ty); $($tail)*}
    };

    (!queries Delete ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty, $CREATE:ty, $UPDATE:ty);
        $ty:ty where delete = | $self:ident, $db:ident | $body:block, $($tail:tt)*) => {

        $crate::macros::create_query!{!impl D1EntityDelete for $ty : $table; | $self, $db | $body}
        $crate::macros::create_queries!{!queries Queries ($cty, $table, $READ_ALL, $READ, $CREATE, $UPDATE, $ty); $($tail)*}
    };

    (!queries Delete ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty, $CREATE:ty, $UPDATE:ty);
        $ty:ty where delete = with $self:ident; [ $($set:tt)* ], $($tail:tt)* ) => {

        $crate::macros::create_query!{!impl D1EntityDelete for $ty : $table; with $self; [ $($set)* ]}
        $crate::macros::create_queries!{!queries Queries ($cty, $table, $READ_ALL, $READ, $CREATE, $UPDATE, $ty); $($tail)*}
    };

    (!queries Queries ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty, $CREATE:ty, $UPDATE:ty, $DELETE:ty); ) => {
        impl $crate::shared::D1EntityQueries<$CREATE, $READ_ALL, $READ, $UPDATE, $DELETE> for $cty { }
    };

    (!create_set $_:ident, $__:ident, $___:ident;) => {};

    (!create_set $out_columns:ident, $out_values:ident, $out_args:ident; $prop:ident = $val:expr, $($tail:tt)*) => {
        $out_columns.push(stringify!($prop));
        $out_values.push("?");
        $out_args.push(($val).into());

        $crate::macros::create_queries!(!create_set $out_columns, $out_values, $out_args; $($tail)*);
    };

    (!create_set $out_columns:ident, $out_values:ident, $out_args:ident; $var:ident?.$prop:ident, $($tail:tt)*) => {
        if let Some(opt) = $var.$prop {
            $out_columns.push(stringify!($prop));
            $out_values.push("?");
            $out_args.push(opt.into());
        }

        $crate::macros::create_queries!(!create_set $out_columns, $out_values, $out_args; $($tail)*);
    };

    (!create_set $out_columns:ident, $out_values:ident, $out_args:ident; $var:ident.$prop:ident, $($tail:tt)*) => {
        $out_columns.push(stringify!($prop));
        $out_values.push("?");
        $out_args.push(($var.$prop).into());

        $crate::macros::create_queries!(!create_set $out_columns, $out_values, $out_args; $($tail)*);
    };
}

#[macro_export]
macro_rules! create_query {
    (!impl D1EntityReadAll for $ty:ty $(: $table:expr)?; | $db:ident | $body:block) => {
        impl $crate::shared::D1EntityReadAll for $ty {
            fn read_all_query($db: ::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                $body
            }
        }
    };

    (!impl D1EntityReadAll for $ty:ty : $table:expr; [ $($select_all:tt)* ]) => {
        impl $crate::shared::D1EntityReadAll for $ty {
            fn read_all_query(d1: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                ::worker::console_log!(concat!(
                    "read_all_query: SELECT ", $crate::macros::create_query!(!vec join [ $($select_all)* ] ", "), " FROM ", $table, " WHERE deleted = FALSE"
                ));
                Ok(d1.prepare(concat!("SELECT ", $crate::macros::create_query!(!vec join [ $($select_all)* ] ", "), " FROM ", $table, " WHERE deleted = FALSE")))
            }
        }
    };

    (!impl D1EntityRead for $ty:ty $(: $table:expr)?; | $self:ident, $db:ident | $body:block) => {
        impl $crate::shared::D1EntityRead for $ty {
            fn read_query(self, $db: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                $body
            }
        }
    };

    (!impl D1EntityRead for $ty:ty : $table:expr; with $self:ident; [ $($set:tt)* ]) => {
        impl $crate::shared::D1EntityRead for $ty {
            fn read_query(self, d1: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                let (query, args) = $crate::macros::create_query!(!select $table; $($set)* );

                ::worker::console_log!("read_query: {query}");
                d1.prepare(query).bind(&args).map_err(|err| format!("{err}"))
            }
        }
    };

    (!impl D1EntityCreate for $ty:ty $(: $table:expr)?; | $self:ident, $db:ident | $body:block) => {
        impl $crate::shared::D1EntityCreate for $ty {
            fn create_query(self, $db: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                $body
            }
        }
    };

    (!impl D1EntityCreate for $ty:ty : $table:expr; with $self:ident; [ $($set:tt)* ]) => {
        impl $crate::shared::D1EntityCreate for $ty {
            fn create_query(self, d1: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                let (query, args) = $crate::macros::create_query!(!create $table; $($set)*);

                ::worker::console_log!("create_query: {}", query);
                d1.prepare(query).bind(&args).map_err(|err| format!("{err}"))
            }
        }
    };

    (!impl D1EntityUpdate for $ty:ty $(: $table:expr)?; | $self:ident, $db:ident | $body:block) => {
        impl $crate::shared::D1EntityUpdate for $ty {
            fn update_query(self, $db: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                $body
            }
        }
    };

    (!impl D1EntityUpdate for $ty:ty : $table:expr; with $self:ident; { where = [ $($where:tt)* ]; set = [ $($set:tt)* ]; }) => {
        impl $crate::shared::D1EntityUpdate for $ty {
            fn update_query(self, d1: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                let (query, args) = $crate::macros::create_query!(!update $table; [ $($where)* ]; [ $($set)* ]);

                ::worker::console_log!("update_query: {}", query);
                ::worker::console_log!("update_query: {:?}", args);
                d1.prepare(query).bind(&args).map_err(|err| format!("{err}"))
            }
        }
    };

    (!impl D1EntityDelete for $ty:ty $(: $table:expr)?; | $self:ident, $db:ident | $body:block) => {
        impl $crate::shared::D1EntityDelete for $ty {
            fn delete_query(self, $db: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                $body
            }
        }
    };

    (!impl D1EntityDelete for $ty:ty : $table:expr; with $self:ident; [ $($set:tt)* ]) => {
        impl $crate::shared::D1EntityDelete for $ty {
            fn delete_query(self, d1: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                let (query, args) = $crate::macros::create_query!(!delete $table; $($set)*);

                ::worker::console_log!("delete_query: {}", query);
                d1.prepare(query).bind(&args).map_err(|err| format!("{err}"))
            }
        }
    };

    (!select $table:expr; $($set:tt)*) => {{
        let (out_query, out_args) = $crate::macros::new_query!(!; $($set)*);

        if out_args.len() == 0 {
            return Err("No properties to select".to_owned());
        }

        (
            format!(concat!("SELECT * FROM ", $table, " WHERE {}"), out_query.join(" AND ")),
            out_args,
        )
    }};

    (!create $table:expr; $($set:tt)*) => {{
        let mut out_columns = vec![];
        let mut out_values = vec![];
        let mut out_args = vec![];

        $crate::macros::create_queries!(!create_set out_columns, out_values, out_args; $($set)*);

        if out_args.len() == 0 {
            return Err("No properties to create".to_owned());
        }

        out_columns.push("created_at");
        out_values.push("?");
        out_args.push((::time::OffsetDateTime::now_utc().unix_timestamp() as f64).into());

        (
            format!(concat!("INSERT INTO ", $table, " ({}) VALUES ({})"), out_columns.join(", "), out_values.join(", ")),
            out_args,
        )
    }};

    (!update $table:expr; [ $($where:tt)* ]; [$($set:tt)*]) => {{
        let (out_query, mut out_args) = $crate::macros::new_query!(!; $($set)*);

        if out_args.len() == 0 {
            return Err("No properties to update".to_owned());
        }

        let out_where = $crate::macros::new_query!(!args = out_args; $($where)*);

        (
            format!(concat!("UPDATE ", $table, " SET {} WHERE {}"), out_query.join(", "), out_where.join(" AND ")),
            out_args,
        )
    }};

    (!delete $table:expr; $($set:tt)*) => {{
        let (out_query, out_args) = $crate::macros::new_query!(!; $($set)*);

        if out_args.len() == 0 {
            return Err("No properties to delete".to_owned());
        }

        (
            format!(concat!("UPDATE ", $table, " SET delete = FALSE WHERE {}"), out_query.join(" AND ")),
            out_args,
        )
    }};

    (!vec join [ $item:ident , $($tail:tt)+ ] $sep:literal) => (concat!( stringify!($item), $sep, $crate::macros::create_query!(!vec join [ $($tail)+ ] $sep)));
    (!vec join [ $item:ident , ] $sep:literal) => (stringify!($item));
    (!vec join [ $item:ident ] $sep:literal) => (stringify!($item));
    (!vec join [ ] $sep:literal) => ("");
}

#[macro_export]
macro_rules! new_query {
    (!;) => ((vec![], vec![]));

    (!; $($tail:tt)+) => {{
        let mut out_query = vec![];
        let mut out_args = vec![];

        $crate::macros::new_query!(!set out_query, out_args; $($tail)*);

        (out_query, out_args)
    }};

    (!query = $out_query:ident;) => (vec![]);

    (!query = $out_query:ident; $($tail:tt)+) => {{
        let mut out_args = vec![];

        $crate::macros::new_query!(!set $out_query, out_args; $($tail)+);

        out_args
    }};

    (!args = $out_args:ident;) => (vec![]);

    (!args = $out_args:ident; $($tail:tt)+) => {{
        let mut out_query = vec![];

        $crate::macros::new_query!(!set out_query, $out_args; $($tail)+);

        out_query
    }};

    (!set $out_query:ident, $out_args:ident;) => {};

    (!set $out_query:ident, $out_args:ident; $prop:ident = $val:expr; $($tail:tt)*) => {
        $out_query.push(concat!(stringify!($prop), " = ?"));
        $out_args.push($val.into());

        $crate::macros::new_query!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; $prop:literal = $val:expr; $($tail:tt)*) => {
        $out_query.push(concat!($prop, " = ?"));
        $out_args.push($val.into());

        $crate::macros::new_query!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; $prop:ident ?= $val:expr; $($tail:tt)*) => {
        if let Some(prop) = $val {
            $out_query.push(concat!(stringify!($prop), " = ?"));
            $out_args.push(prop.into());
        }

        $crate::macros::new_query!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; $prop:literal ?= $val:expr; $($tail:tt)*) => {
        if let Some(prop) = $val {
            $out_query.push(concat!($prop, " = ?"));
            $out_args.push(prop.into());
        }

        $crate::macros::new_query!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; &$prop:ident ?= $val:expr; $($tail:tt)*) => {
        if let Some(ref prop) = $val {
            $out_query.push(concat!(stringify!($prop), " = ?"));
            $out_args.push(prop.into());
        }

        $crate::macros::new_query!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; $var:ident.$prop:ident; $($tail:tt)*) => {
        $out_query.push(concat!(stringify!($prop), " = ?"));
        $out_args.push(($var.$prop).into());

        $crate::macros::new_query!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; &$var:ident.$prop:ident; $($tail:tt)*) => {
        $out_query.push(concat!(stringify!($prop), " = ?"));
        $out_args.push((&$var.$prop).into());

        $crate::macros::new_query!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; $var:ident?.$prop:ident; $($tail:tt)*) => {
        if let Some(prop) = $var.$prop {
            $out_query.push(concat!(stringify!($prop), " = ?"));
            $out_args.push(prop.into());
        }

        $crate::macros::new_query!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; &$var:ident?.$prop:ident; $($tail:tt)*) => {
        if let Some(ref prop) = $var.$prop {
            $out_query.push(concat!(stringify!($prop), " = ?"));
            $out_args.push(prop.into());
        }

        $crate::macros::new_query!(!set $out_query, $out_args; $($tail)*);
    };
}

pub use create_queries;
pub use create_query;
pub use new_query;
