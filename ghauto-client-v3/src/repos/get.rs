imports!();
use crate::client::GetQueryBuilder;

new_type!(
    Labels
    LabelsName
    Owner
    Repo
    Repos
);

from!(
    @Labels
        => LabelsName

    @GetQueryBuilder
        -> Repos = "repos"
    @Owner
        => Repo
    @Repo
        -> Labels = "labels"
    @Repos
        => Owner
);

impl_macro!(
    @Labels
        |
        |=> labelname -> LabelsName = labelname
    @Owner
        |
        |=> repo -> Repo = repo_str
    @Repo
        |=> labels -> Labels
        |
    @Repos
        |
        |=> owner -> Owner = username_str
);

exec!(Labels);
exec!(Repo);
