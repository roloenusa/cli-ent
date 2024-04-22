use octocrab::Octocrab;

#[allow(dead_code)]
pub async fn octo_binding() {
  let token = "<secret>".to_string();
  let octocrab = Octocrab::builder().personal_token(token).build().unwrap();
  let repo = octocrab.repos("rust-lang", "rust").get().await;
  let repo = repo.unwrap();
  println!("{:#?}", repo.ssh_url);
}