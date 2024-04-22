use reqwest::Method;

pub fn ghlib_binding() {
  // curl -L \
  // -H "Accept: application/vnd.github+json" \
  // -H "Authorization: Bearer <YOUR-TOKEN>" \
  // -H "X-GitHub-Api-Version: 2022-11-28" \
  // https://api.github.com/orgs/ORG/teams/TEAM_SLUG/repos

  let client = reqwest::blocking::Client::new();
  let res = client.request(Method::GET, "https://api.github.com/orgs/rust-lang/repos")
    .header("user-agent", env!("CARGO_PKG_NAME"))
    .header("Accept", "application/vnd.github+json")
    .header("Authorization", "Bearer <secret>")
    .header("X-GitHub-Api-Version", "2022-11-28")
    .send();

  match res {
    Ok(r) => {
      println!("{:#?}", r);
      println!("{:#?}", r.text().unwrap());
    },
    Err(e) => println!("Error: {:#?}", e),
  };



}