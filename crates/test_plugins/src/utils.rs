use colored::Colorize;
use similar::ChangeTag;

pub fn show_diff(expected: &str, res: &str) {
  let diff = similar::TextDiff::from_lines(expected.trim(), res.trim());

  for change in diff.iter_all_changes() {
    let sign = match change.tag() {
      ChangeTag::Delete => format!("-{}", change.value().red()),
      ChangeTag::Insert => format!("+{}", change.value().green()),
      ChangeTag::Equal => format!(" {}", change.value()),
    };
    print!("{}", sign);
  }
  println!("");
}
