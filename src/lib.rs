pub use lazy_static::lazy_static;
pub use paste::paste;
pub use pgw::{Error, IntoStatement, Pg, Row, Sql, ToSql, ToStatement};

#[macro_export]
macro_rules! q {
  ($db:ident, $name:ident) => {
    $crate::lazy_static! {
      static ref $db: $crate::Pg = $crate::Pg::new_with_env(format!("{}_URI",stringify!($db)));
    }
    $crate::paste! {
      q!($db, $name, query, Vec<$crate::Row>);
      q!($db, [<$name 1>], query_one, $crate::Row);
      q!($db, [<$name 01>], query_opt, Option<$crate::Row>);
    }
  };
  ($db:ident,$name:ident,$func:ident,$rt:ty) => {
    #[allow(non_snake_case)]
    pub async fn $name<T: $crate::ToStatement>(
      statement: impl $crate::IntoStatement<T>,
      params: &[&(dyn $crate::ToSql + Sync)],
    ) -> Result<$rt, $crate::Error> {
      (*$db).$func(statement, params).await
    }
  };
}
