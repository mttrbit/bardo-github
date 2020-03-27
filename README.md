# Github Repo Automation
> A set of tools to automate multiple GitHub repository management.

As we publish a couple of libraries to multiple repositories under crvshlab, we need a set of small tools to perform management of those repositories, such as creating new repositories, updating continuous integration setup, updating dependencies, and so on.

This repository contains some scripts that may be useful for these kind of tasks.

# How to run
The project is still very early stage so you need to expect that things change. At the moment there is no executable and the project is run using `cargo`, e.g.

`GITHUB_CLIENT_ID=xxx GITHUB_CLIENT_SECRET=xxx cargo run bardo repo` to generate authorization url.

Once stable features are available, the section will be updated.


# Installation

You need to make your own `credentials` and put your github client id, and github client secret there. You can set the path to the configuruation folder containing the credentials file with the `BARDO_CONFIG_HOME` environment variable:
```
$ cat /Users/seka/.config/bardo/gh/credentials
---
[default]
bardo_access_token = "YOUR_ACCESS_TOKEN"
bardo_client_id = "YOUR_CLIENT_ID"

```

You need to make your own `config` file. You can set the path to the configuruation folder containing the config file with the `BARDO_CONFIG_HOME` environment variable:
```
[default]
clone_path = "/Users/seka/bardo_test"
repositories = [
  { org = "YOUR_ORG", name = "REPO_NAME" }
,
]

```

# Usage (Planned)
```
bardo gh pr [ls, approve]

bardo gh issue [ls]

bardo gh repo [view, create, clone, fork, add, rename]

bardo gh help
  

```



# Discussion

## Why using Rust?
Rust seems to be a great language and I simply want to take the chance to learn Rust.

## Why not using Github Cli?
Github Cli is an awesome tool but it is still beta. Furthermore it can only fetch repository information on a per repo basis. However, we would like to fetch information about a set of projects and potentially update a set of projects as well. 

