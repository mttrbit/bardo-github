imports!();
use crate::client::PostQueryBuilder;

new_type!(
    Assignees
    Git
    Issues
    IssuesNumber
    Owner
    Pulls
    PullsNumber
    Ref
    Refs
    Repo
    Repos
    RequestedReviewers
    Sha
);

from!(
    @Git
        -> Refs = "refs"
    @Issues
        => IssuesNumber
    @IssuesNumber
        -> Assignees = "assignees"
    @Owner
        => Repo
    @PostQueryBuilder
        -> Repos = "repos"
    @Pulls
        => PullsNumber
    @PullsNumber
        -> RequestedReviewers = "requested_reviewers"
    @Repo
        -> Issues = "issues"
        -> Git = "git"
        -> Pulls = "pulls"
    @Repos
        => Owner
);

impl_macro!(
    @Issues
        |
        |=> issues_number -> IssuesNumber = issues_number
    @IssuesNumber
        |=> assignees -> Assignees
        |
    @Git
        |=> refs -> Refs
        |
    @Owner
        |
        |=> repo -> Repo = repo_str
    @Pulls
        |
        |=> pulls_number -> PullsNumber = pulls_number
    @PullsNumber
        |=> requested_reviewers -> RequestedReviewers
        |
    @Refs
        |
    @Repo
        |=> pulls -> Pulls
        |=> git -> Git
        |=> issues -> Issues
        |
    @Repos
        |
        |=> owner -> Owner = username_str
);

exec!(Assignees);
exec!(Pulls);
exec!(Refs);
exec!(RequestedReviewers);
