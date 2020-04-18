imports!();
use crate::client::GetQueryBuilder;

new_type!(
    Commits
    Contents
    Issues
    IssuesPage
    IssuesState
    IssuesNumber
    Labels
    LabelsName
    Owner
    Path
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
    @Contents
        => Path
    @Issues
    @Issues
        => IssuesNumber
    @Issues
        ?> IssuesPage = "page"
    @Labels
        => LabelsName
    @GetQueryBuilder
        -> Repos = "repos"
    @Owner
        => Repo
    @Path
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
    @Repo
        -> Contents = "contents"
    @Repos
        => Owner
    @Reference
);

impl_macro!(
    @Commits
        |
        |=> reference -> Reference = ref_str
    @Contents
        |
        |=> path -> Path = path
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
        |=> contents -> Contents
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
exec!(Path);
exec!(Pulls);
exec!(PullsNumber);
exec!(PullsPage);
exec!(PullsState);
exec!(Repo);
exec!(Reference);
