use async_trait::async_trait;

#[async_trait]
pub trait Generator {
  fn name<'a>( &self ) -> &'a str;
  async fn call( &self
               , prompt: &str
               , fmode: bool
               , personality: &str ) -> anyhow::Result<String>;
}
