imports!();
use crate::client::PostQueryBuilder;

new_type!(
    Assignees
    Git
    Issues
    IssuesNumber
    Owner
    Pulls
    Ref
    Refs
    Repo
    Repos
    Sha
);

from!(
    @Git
        -> Refs = "refs"
    @Issues
        -> Assignees = "assignees"
    @Owner
        => Repo
    @PostQueryBuilder
        -> Repos = "repos"
    @Repo
        -> Issues = "issues"
        -> Git = "git"
        -> Pulls = "pulls"
    @Repos
        => Owner
);

impl_macro!(
    @Issues
        |=> assignees -> Assignees
        |
    @Git
        |=> refs -> Refs
        |
    @Owner
        |
        |=> repo -> Repo = repo_str
    @Refs
        |
    @Repo
        |=> pulls -> Pulls
        |=> git -> Git
        |
    @Repo
        |
        |=> issues -> Issues = issues_number
    @Repos
        |
        |=> owner -> Owner = username_str
);

exec!(Assignees);
exec!(Pulls);
exec!(Refs);
