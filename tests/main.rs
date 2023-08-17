use lazy_static::lazy_static;
use pgm::{q, Sql};

q!(PG, Q);

lazy_static! {
  static ref OID: Sql = PG.sql("SELECT oid FROM pg_catalog.pg_namespace LIMIT 1");
}

#[tokio::test]
async fn main() -> anyhow::Result<()> {
  let r = Q1(&*OID, &[]).await?;
  dbg!(r);
  Ok(())
}
