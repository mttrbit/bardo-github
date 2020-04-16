imports!();
use crate::client::GetQueryBuilder;

new_type!(
    Commits
    Issues
    IssuesPage
    IssuesState
    IssuesNumber
    Labels
    LabelsName
    Owner
    Pulls
    PullsNumber
    PullsPage
    PullsState
    Reference
    Repo
    Repos
);

from!(
    @Commits
        => Reference
    @Issues
    @Issues
        => IssuesNumber
    @Issues
        ?> IssuesPage = "page"
    @IssuesNumber
    @Labels
        => LabelsName
    @GetQueryBuilder
        -> Repos = "repos"
    @Owner
        => Repo
    @Pulls
        => PullsNumber
    @Pulls
        ?> PullsPage = "page"
    @PullsNumber
    @Repo
        -> Issues = "issues"
        -> Pulls = "pulls"
    @Repo
        -> Labels = "labels"
    @Repo
        -> Commits = "commits"
    @Repos
        => Owner
    @Reference
);

impl_macro!(
    @Commits
        |
        |=> reference -> Reference = ref_str
    @Issues
        |
        |=> number -> IssuesNumber = issue_number
        |?> page -> IssuesPage = page
    @Labels
        |
        |=> labelname -> LabelsName = labelname
    @Owner
        |
        |=> repo -> Repo = repo_str
    @Pulls
        |
        |=> number -> PullsNumber = pulls_number
        |?> page -> PullsPage = page
    @Repo
        |=> issues -> Issues
        |=> labels -> Labels
        |=> pulls -> Pulls
        |=> commits -> Commits
        |
    @Repos
        |
        |=> owner -> Owner = username_str
    @Reference
        |
);

exec!(Issues);
exec!(IssuesPage);
exec!(IssuesState);
exec!(IssuesNumber);
exec!(Labels);
exec!(Pulls);
exec!(PullsNumber);
exec!(PullsPage);
exec!(PullsState);
exec!(Repo);
exec!(Reference);
