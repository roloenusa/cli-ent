#[allow(dead_code)]
use std::io::{self, Write};
use chrono::{DateTime, Utc};
use git2::{AnnotatedCommit, Commit, ObjectType, Repository};


pub fn git_binding() {
  // Get the repo
  let repo = match Repository::open("/Users/roloenusa/projects/personal/mort") {
      Ok(repo) => repo,
      Err(e) => panic!("failed to open: {}", e),
  };
  println!("{} state={:?}", repo.path().display(), repo.state());

  // Previous commit
  let commit = find_last_commit(&repo).expect("Wrong commit");
  display_commit(&commit);

  pull(&repo);
  checkout(&repo);

  // let remote_branch = "master";
  // let fetch_commit = fetch_commit(&repo).unwrap();
  // // println!("---- {:#?}", "hello");
  // do_merge(&repo, &remote_branch, fetch_commit);

  // update(&repo);

  // Newest commit
  let commit = find_last_commit(&repo).expect("Wrong commit");
  display_commit(&commit);
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
  let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
  obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn display_commit(commit: &Commit) {
  let timestamp = commit.time().seconds();
  let tm: DateTime<Utc> = DateTime::from_timestamp(timestamp, 0).unwrap();
  println!("commit {}\nAuthor: {}\nDate:   {}\n\n\t{}",
           commit.id(),
           commit.author(),
           tm.to_rfc2822(),
           commit.message().unwrap_or("no commit message"));
}

fn fetch_commit(repo: &Repository) -> Result<AnnotatedCommit<'_>, git2::Error> {
  let mut callbacks = git2::RemoteCallbacks::new();
  callbacks.credentials(git_credentials_callback);
  callbacks.transfer_progress(|stats| {
      if stats.received_objects() == stats.total_objects() {
          print!(
              "Resolving deltas {}/{}\r",
              stats.indexed_deltas(),
              stats.total_deltas()
          );
      } else if stats.total_objects() > 0 {
          print!(
              "Received {}/{} objects ({}) in {} bytes\r",
              stats.received_objects(),
              stats.total_objects(),
              stats.indexed_objects(),
              stats.received_bytes()
          );
      }
      io::stdout().flush().unwrap();
      true
  });

  let mut opts = git2::FetchOptions::new();
  opts.remote_callbacks(callbacks);

  let mut remote = match repo.find_remote("origin") {
      Ok(r) => r,
      Err(e) => panic!("{e}"),
  };
  remote.fetch(&["main"], std::option::Option::Some(&mut opts), None).unwrap();

  // If there are local objects (we got a thin pack), then tell the user
  // how many objects we saved from having to cross the network.
  let stats = remote.stats();
  if stats.local_objects() > 0 {
      println!(
          "\rReceived {}/{} objects in {} bytes (used {} local \
           objects)",
          stats.indexed_objects(),
          stats.total_objects(),
          stats.received_bytes(),
          stats.local_objects()
      );
  } else {
      println!(
          "\rReceived {}/{} objects in {} bytes",
          stats.indexed_objects(),
          stats.total_objects(),
          stats.received_bytes()
      );
  }

  let fetch_head = repo.find_reference("FETCH_HEAD")?;
  repo.reference_to_annotated_commit(&fetch_head)
}

pub fn git_credentials_callback(
  _user: &str,
  _user_from_url: Option<&str>,
  _cred: git2::CredentialType,
) -> Result<git2::Cred, git2::Error> {
  let user = _user_from_url.unwrap_or("git");

  if _cred.contains(git2::CredentialType::USERNAME) {
      return git2::Cred::username(user);
  }


  println!("authenticate with user {} and private key located in", user);
  git2::Cred::ssh_key(user, None, std::path::Path::new("/Users/roloenusa/.ssh/id_rsa"), None)
}

#[allow(dead_code)]
fn fast_forward(
  repo: &Repository,
  lb: &mut git2::Reference,
  rc: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
  let name = match lb.name() {
      Some(s) => s.to_string(),
      None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
  };
  let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
  println!("{}", msg);
  lb.set_target(rc.id(), &msg)?;
  repo.set_head(&name)?;
  repo.checkout_head(Some(
      git2::build::CheckoutBuilder::default()
          // For some reason the force is required to make the working directory actually get updated
          // I suspect we should be adding some logic to handle dirty working directory states
          // but this is just an example so maybe not.
          .force(),
  ))?;
  Ok(())
}

#[allow(dead_code)]
fn normal_merge(
  repo: &Repository,
  local: &git2::AnnotatedCommit,
  remote: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
  let local_tree = repo.find_commit(local.id())?.tree()?;
  let remote_tree = repo.find_commit(remote.id())?.tree()?;
  let ancestor = repo
      .find_commit(repo.merge_base(local.id(), remote.id())?)?
      .tree()?;
  let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

  if idx.has_conflicts() {
      println!("Merge conflicts detected...");
      repo.checkout_index(Some(&mut idx), None)?;
      return Ok(());
  }
  let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
  // now create the merge commit
  let msg = format!("Merge: {} into {}", remote.id(), local.id());
  let sig = repo.signature()?;
  let local_commit = repo.find_commit(local.id())?;
  let remote_commit = repo.find_commit(remote.id())?;
  // Do our merge commit and set current branch head to that commit.
  let _merge_commit = repo.commit(
      Some("HEAD"),
      &sig,
      &sig,
      &msg,
      &result_tree,
      &[&local_commit, &remote_commit],
  )?;
  // Set working tree to match head.
  repo.checkout_head(None)?;
  Ok(())
}

#[allow(dead_code)]
fn do_merge<'a>(
  repo: &'a Repository,
  remote_branch: &str,
  fetch_commit: git2::AnnotatedCommit<'a>,
) -> Result<(), git2::Error> {
  // 1. do a merge analysis
  let analysis = repo.merge_analysis(&[&fetch_commit])?;

  // 2. Do the appropriate merge
  if analysis.0.is_fast_forward() {
      println!("Doing a fast forward");
      // do a fast forward
      let refname = format!("refs/heads/{}", remote_branch);
      match repo.find_reference(&refname) {
          Ok(mut r) => {
              fast_forward(repo, &mut r, &fetch_commit)?;
          }
          Err(_) => {
              // The branch doesn't exist so just set the reference to the
              // commit directly. Usually this is because you are pulling
              // into an empty repository.
              repo.reference(
                  &refname,
                  fetch_commit.id(),
                  true,
                  &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
              )?;
              repo.set_head(&refname)?;
              repo.checkout_head(Some(
                  git2::build::CheckoutBuilder::default()
                      .allow_conflicts(true)
                      .conflict_style_merge(true)
                      .force(),
              ))?;
          }
      };
  } else if analysis.0.is_normal() {
      // do a normal merge
      let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
      normal_merge(&repo, &head_commit, &fetch_commit)?;
  } else {
      println!("Nothing to do...");
  }
  Ok(())
}

fn pull(repo: &Repository) {
  let mut callbacks = git2::RemoteCallbacks::new();
  callbacks.credentials(git_credentials_callback);
  callbacks.transfer_progress(|stats| {
      if stats.received_objects() == stats.total_objects() {
          print!(
              "Resolving deltas {}/{}\r",
              stats.indexed_deltas(),
              stats.total_deltas()
          );
      } else if stats.total_objects() > 0 {
          print!(
              "Received {}/{} objects ({}) in {} bytes\r",
              stats.received_objects(),
              stats.total_objects(),
              stats.indexed_objects(),
              stats.received_bytes()
          );
      }
      io::stdout().flush().unwrap();
      true
  });

  let mut opts = git2::FetchOptions::new();
  opts.remote_callbacks(callbacks);

  let mut remote = match repo.find_remote("origin") {
      Ok(r) => r,
      Err(e) => panic!("{e}"),
  };
  remote.fetch(&["main"], std::option::Option::Some(&mut opts), None).unwrap();
}

fn checkout(repo: &Repository) {
  let refname = format!("refs/heads/{}", "main");

  let fetch_commit = fetch_commit(repo).unwrap();
  repo.reference(
      &refname,
      fetch_commit.id(),
      true,
      &format!("Setting {} to {}", "main", fetch_commit.id()),
  ).unwrap();


  repo.set_head(&refname).unwrap();
  repo.checkout_head(Some(
      git2::build::CheckoutBuilder::default()
          .allow_conflicts(true)
          .conflict_style_merge(true)
          .force(),
  )).unwrap();
}