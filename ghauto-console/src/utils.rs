pub fn pick_repo<'a>(args: &'a Vec<Vec<&'a str>>) -> Option<(&'a str, &'a str)> {
    for v in args {
        if v[0] == "repo" {
            let mut split: std::str::Split<&str> = v[1].split("/");
            let org = split.next().expect("organisation missing");
            let name = split.next().expect("name missing");
            return Some((org, name));
        }
    }

    return None;
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

pub fn pick_all<'a>(args: &'a Vec<Vec<&'a str>>) -> bool {
    for v in args {
        if v[0] == "ALL" {
            return true;
        }
    }

    false
}
