imports!();
use crate::client::GetQueryBuilder;

new_type!(
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
    Repo
    Repos
);

from!(
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
    @Repos
        => Owner
);

impl_macro!(
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
        |
    @Repos
        |
        |=> owner -> Owner = username_str
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
