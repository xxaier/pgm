#[macro_export]
macro_rules! q {
  ($db:ident, $name:ident) => {
    lazy_static::lazy_static! {
      static ref $db: Pg = Pg::new_with_env(format!("{}_URI",stringify!($db)));
    }
    paste! {
      q!($db, $name, query, Vec<Row>);
      q!($db, [<$name 1>], query_one, Row);
      q!($db, [<$name 01>], query_opt, Option<Row>);
    }
  };
  ($db:ident,$name:ident,$func:ident,$rt:ty) => {
    #[allow(non_snake_case)]
    pub async fn $name<T: ToStatement>(
      statement: impl IntoStatement<T>,
      params: &[&(dyn ToSql + Sync)],
    ) -> Result<$rt, Error> {
      $db.$func(statement, params).await
    }
  };
}
