use async_trait::async_trait;

#[async_trait]
pub trait Generator {
  fn name<'a>( &self ) -> &'a str;
  fn enabled_for_multigen( &self ) -> bool;
  async fn call( &self
               , prompt: &str
               , fmode: bool
               , personality: &str ) -> anyhow::Result<String>;
}
