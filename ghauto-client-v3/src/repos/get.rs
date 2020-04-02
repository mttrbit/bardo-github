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
    @Repo
        -> Issues = "issues"
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
    @Repo
        |=> issues -> Issues
        |=> labels -> Labels
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
exec!(Repo);
