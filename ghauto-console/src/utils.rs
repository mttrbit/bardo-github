pub fn pick_repo<'a>(args: &'a Vec<Vec<&'a str>>) -> Option<(&'a str, &'a str)> {
    for v in args {
        if v[0] == "REPO" {
            let mut split: std::str::Split<&str> = v[1].split("/");
            let org = split.next().expect("organisation missing");
            let name = split.next().expect("name missing");
            return Some((org, name));
        }
    }

    None
}

pub fn maybe_filter_repo<'a>(
    repo: &'a config::config::Repository,
    arg: &'a Option<(&str, &str)>,
) -> bool {
    match Some((arg, repo.org(), repo.name())) {
        Some((Some((org, name)), r_org, Some(r_name))) => {
            r_org.0 == org.to_string() && r_name.0 == name.to_string()
        }
        _ => true,
    }
}

pub fn print_all<'a>(args: &'a Vec<Vec<&'a str>>) -> bool {
    for v in args {
        if v[0] == "ALL" {
            return true;
        }
    }

    false
}

pub fn pick_command<'a>(args: &'a Vec<Vec<&'a str>>) -> Option<&'a str> {
    for v in args {
        if v[0] == "CMD" {
            return Some(v[1]);
        }
    }

    None
}

pub fn pick_branch<'a>(args: &'a Vec<Vec<&'a str>>) -> Option<&'a str> {
    for v in args {
        if v[0] == "BRANCH" {
            return Some(v[1]);
        }
    }

    None
}

pub fn pick_message<'a>(args: &'a Vec<Vec<&'a str>>) -> Option<&'a str> {
    for v in args {
        if v[0] == "MESSAGE" {
            return Some(v[1]);
        }
    }

    None
}

pub fn pick_comment<'a>(args: &'a Vec<Vec<&'a str>>) -> Option<&'a str> {
    for v in args {
        if v[0] == "COMMENT" {
            return Some(v[1]);
        }
    }

    None
}

pub fn pick_reviewers<'a>(args: &'a Vec<Vec<&'a str>>) -> Option<Vec<String>> {
    for v in args {
        if v[0] == "REVIEWERS" {
            let mut split: std::str::Split<&str> = v[1].split("/");
            let mut reviewers = Vec::new();
            for r in split.next() {
                reviewers.push(r.to_string())
            }

            return Some(reviewers);
        }
    }

    None
}
