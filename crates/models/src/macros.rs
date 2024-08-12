#[macro_export]
macro_rules! create_queries {
    (
        $(table = $table:literal;)?
        $ty:ty where select_all = $($select_all:tt)*
    ) => {
        $crate::macros::create_queries!{!queries 0 ($ty, $crate::macros::create_queries!(!table $ty, $($table)?)); $($select_all)*}
    };

    (!table $ty:ty,) => (stringify!($ty));
    (!table $ty:ty, $table:literal) => (stringify!($table));

    (!queries 0 ($ty:ty, $table:expr); | $db:ident | $body:block, $($tail:tt)*) => {
        impl $create::shared::D1EntityReadAll for $ty {
            fn read_all_query($db: ::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                $body
            }
        }

        $crate::macros::create_queries!{!queries 5 ($ty, $table, $ty); $($tail)*}
    };

    (!queries 0 ($ty:ty, $table:expr); $select_all:literal, $($tail:tt)*) => {
        impl $crate::shared::D1EntityReadAll for $ty {
            fn read_all_query(d1: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                ::worker::console_log!(concat!("read_all_query: SELECT ", $select_all, " FROM ", $table, " WHERE deleted = FALSE"));
                Ok(d1.prepare(concat!("SELECT ", $select_all, " FROM ", $table, " WHERE deleted = FALSE")))
            }
        }

        $crate::macros::create_queries!{!queries 5 ($ty, $table, $ty); $($tail)*}
    };

    (!queries 5 ($cty:ty, $table:expr, $READ_ALL:ty); $ty:ty where select = | $self:ident, $db:ident | $body:block, $($tail:tt)*) => {
        impl $crate::shared::D1EntityRead for $ty {
            fn read_query(self, $db: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;

                $body
            }
        }


        $crate::macros::create_queries!{!queries 1 ($cty, $table, $READ_ALL, $ty); $($tail)*}
    };

    (!queries 5 ($cty:ty, $table:expr, $READ_ALL:ty); $ty:ty where select = with $self:ident; [$($set:tt)+], $($tail:tt)*) => {
        impl $crate::shared::D1EntityRead for $ty {
            fn read_query(self, d1: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                let (query, args) = $crate::macros::create_queries!(!select ($table);
                    [ $($set)+ ];
                );

                ::worker::console_log!("select_query: {}", query);
                d1.prepare(query).bind(&args).map_err(|err| format!("{err}"))
            }
        }

        $crate::macros::create_queries!{!queries 1 ($cty, $table, $READ_ALL, $ty); $($tail)*}
    };

    (!select ($table:expr); [$($set:tt)+];) => {{
        let (out_query, out_args) =
            $crate::macros::create_queries!(!new_query; $($set)+);

        if out_args.len() == 0 {
            return Err("No properties to select".to_owned());
        }

        (
            format!(concat!("SELECT * FROM ", $table, " WHERE {}"), out_query.join(" AND ")),
            out_args,
        )
    }};

    (!queries 1 ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty); $ty:ty where create = | $self:ident, $db:ident | $body:block, $($tail:tt)*) => {
        impl $crate::shared::D1EntityCreate for $ty {
            fn create_query(self, $db: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                $body
            }
        }


        $crate::macros::create_queries!{!queries 2 ($cty, $table, $READ_ALL, $READ, $ty); $($tail)*}
    };

    (!queries 1 ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty);
        $ty:ty where create = with $self:ident; [ $($set:tt)+ ], $($tail:tt)*) => {
        impl $crate::shared::D1EntityCreate for $ty {
            fn create_query(self, d1: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                let (query, args) = $crate::macros::create_queries!(!create ($table);
                    set = [ $($set)+ ];
                );

                ::worker::console_log!("create_query: {}", query);
                d1.prepare(query).bind(&args).map_err(|err| format!("{err}"))
            }
        }

        $crate::macros::create_queries!{!queries 2 ($cty, $table, $READ_ALL, $READ, $ty); $($tail)*}
    };

    (!create ($table:expr); set = [$($set:tt)+];) => {{
        let mut out_columns = vec![];
        let mut out_values = vec![];
        let mut out_args = vec![];

        $crate::macros::create_queries!(!create_set out_columns, out_values, out_args; $($set)+);

        if out_args.len() == 0 {
            return Err("No properties to create".to_owned());
        }

        out_columns.push("created_at");
        out_values.push("?");
        out_args.push((::time::OffsetDateTime::now_utc().unix_timestamp() as f64).into());

        (
            format!(concat!("INSERT INTO ", $table, "({}) VALUES ({})"), out_columns.join(", "), out_values.join(", ")),
            out_args,
        )
    }};

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

    (!queries 2 ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty, $CREATE:ty); $ty:ty where update = | $db:ident | $body:block, $($tail:tt)*) => {
        impl $create::shared::D1EntityUpdate for $ty {
            fn update_query($db: ::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                $body
            }
        }


        $crate::macros::create_queries!{!queries 3 ($cty, $table, $READ_ALL, $READ, $CREATE, $ty); $($tail)*}
    };

    (!queries 2 ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty, $CREATE:ty);
        $ty:ty where update = with $self:ident; {
            where = [ $($where:tt)+ ];
            set = [ $($set:tt)+ ];
        }, $($tail:tt)*) => {
        impl $crate::shared::D1EntityUpdate for $ty {
            fn update_query(self, d1: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                let (query, args) = $crate::macros::create_queries!(!update ($table);
                    where = [ $($where)+ ];
                    set = [ $($set)+ ];
                );

                ::worker::console_log!("update_query: {}", query);
                d1.prepare(query).bind(&args).map_err(|err| format!("{err}"))
            }
        }

        $crate::macros::create_queries!{!queries 3 ($cty, $table, $READ_ALL, $READ, $CREATE, $ty); $($tail)*}
    };

    (!update ($table:expr); where = [ $($where:tt)+ ]; set = [$($set:tt)+];) => {{
        let (out_query, mut out_args) =
            $crate::macros::create_queries!(!new_query; $($set)+);

        if out_args.len() == 0 {
            return Err("No properties to update".to_owned());
        }

        let out_where =
            $crate::macros::create_queries!(!new_query args = out_args; $($where)+);

        (
            format!(concat!("UPDATE ", $table, " SET {} WHERE {}"), out_query.join(", "), out_where.join(" AND ")),
            out_args,
        )
    }};


    (!queries 3 ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty, $CREATE:ty, $UPDATE:ty); $ty:ty where delete = | $self:ident, $db:ident | $body:block, $($tail:tt)*) => {
        impl $crate::shared::D1EntityDelete for $ty {
            fn delete_query(self, $db: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                $body
            }
        }

        $crate::macros::create_queries!{!queries 4 ($cty, $table, $READ_ALL, $READ, $CREATE, $UPDATE, $ty); $($tail)*}
    };

    (!queries 3 ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty, $CREATE:ty, $UPDATE:ty); $ty:ty where delete = with $self:ident; [ $($set:tt)+ ], $($tail:tt)* ) => {
        impl $crate::shared::D1EntityDelete for $ty {
            fn delete_query(self, d1: &::worker::D1Database) -> ::std::result::Result<::worker::D1PreparedStatement, String> {
                let $self = self;
                let (query, args) = $crate::macros::create_queries!(!delete ($table);
                    [ $($set)+ ];
                );

                d1.prepare(query).bind(&args).map_err(|err| format!("{err}"))
            }
        }

        $crate::macros::create_queries!{!queries 4 ($cty, $table, $READ_ALL, $READ, $CREATE, $UPDATE, $ty); $($tail)*}
    };

    (!delete ($table:expr); [$($set:tt)+];) => {{
        let (out_query, out_args) =
            $crate::macros::create_queries!(!new_query; $($set)+);

        if out_args.len() == 0 {
            return Err("No properties to delete".to_owned());
        }

        (
            format!(concat!("UPDATE ", $table, " SET delete = FALSE WHERE {}"), out_query.join(" AND ")),
            out_args,
        )
    }};

    (!queries 4 ($cty:ty, $table:expr, $READ_ALL:ty, $READ:ty, $CREATE:ty, $UPDATE:ty, $DELETE:ty); ) => {
        impl $crate::shared::D1EntityQueries<$CREATE, $READ_ALL, $READ, $UPDATE, $DELETE> for $cty { }
    };

    (!new_query ; $($tail:tt)*) => {{
        let mut out_query = vec![];
        let mut out_args = vec![];

        $crate::macros::create_queries!(!set out_query, out_args; $($tail)+);

        (out_query, out_args)
    }};

    (!new_query query = $out_query:ident; $($tail:tt)*) => {{
        let mut out_args = vec![];

        $crate::macros::create_queries!(!set $out_query, out_args; $($tail)+);

        out_args
    }};

    (!new_query args = $out_args:ident; $($tail:tt)*) => {{
        let mut out_query = vec![];

        $crate::macros::create_queries!(!set out_query, $out_args; $($tail)+);

        out_query
    }};

    (!set $out_query:ident, $out_args:ident;) => {};

    (!set $out_query:ident, $out_args:ident; $prop:ident = $val:expr; $($tail:tt)*) => {
        $out_query.push(concat!(stringify!($prop), " = ?"));
        $out_args.push($val.into());

        $crate::macros::create_queries!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; $prop:ident = $val:expr; $($tail:tt)*) => {
        $out_query.push(concat!(stringify!($prop), " = ?"));
        $out_args.push((&$val).into());

        $crate::macros::create_queries!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; $prop:ident ?= $val:expr; $($tail:tt)*) => {
        if let Some(prop) = $val {
            $out_query.push(concat!(stringify!($prop), " = ?"));
            $out_args.push(prop.into());
        }

        $crate::macros::create_queries!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; &$prop:ident ?= $val:expr; $($tail:tt)*) => {
        if let Some(ref prop) = $val {
            $out_query.push(concat!(stringify!($prop), " = ?"));
            $out_args.push(prop.into());
        }

        $crate::macros::create_queries!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; $var:ident.$prop:ident; $($tail:tt)*) => {
        $out_query.push(concat!(stringify!($prop), " = ?"));
        $out_args.push(($var.$prop).into());

        $crate::macros::create_queries!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; &$var:ident.$prop:ident; $($tail:tt)*) => {
        $out_query.push(concat!(stringify!($prop), " = ?"));
        $out_args.push((&$var.$prop).into());

        $crate::macros::create_queries!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; $var:ident?.$prop:ident; $($tail:tt)*) => {
        if let Some(prop) = $var.$prop {
            $out_query.push(concat!(stringify!($prop), " = ?"));
            $out_args.push(prop.into());
        }

        $crate::macros::create_queries!(!set $out_query, $out_args; $($tail)*);
    };

    (!set $out_query:ident, $out_args:ident; &$var:ident?.$prop:ident; $($tail:tt)*) => {
        if let Some(ref prop) = $var.$prop {
            $out_query.push(concat!(stringify!($prop), " = ?"));
            $out_args.push(prop.into());
        }

        $crate::macros::create_queries!(!set $out_query, $out_args; $($tail)*);
    };
}

pub use create_queries;
