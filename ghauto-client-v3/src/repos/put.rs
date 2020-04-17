imports!();
use crate::client::PutQueryBuilder;

new_type!(
    Contents
    Owner
    Path
    Repo
    Repos
);

from!(
    @Contents
        => Path
    @PutQueryBuilder
        -> Repos = "repos"
    @Owner
        => Repo
    @Path
    @Repo
        -> Contents = "contents"
    @Repos
        => Owner
);

impl_macro!(
    @Contents
        |
        |=> path -> Path = path
    @Owner
        |
        |=> repo -> Repo = repo_str
    @Repo
        |=> contents -> Contents
        |
    @Repos
        |
        |=> owner -> Owner = username_str
);

exec!(Path);
