# Backoffice Github Repo Automation
> A set of tools to automate multiple GitHub repository management.

As we publish a couple of libraries to multiple repositories under crvshlab, we need a set of small tools to perform management of those repositories, such as creating new repositories, updating continuous integration setup, updating dependencies, and so on.

This repository contains some scripts that may be useful for these kind of tasks.

# Usage

Run via
`GITHUB_CLIENT_ID=xxx GITHUB_CLIENT_SECRET=xxx cargo run bardo repo`
to generate authorization url.

# Discussion

## Why using Rust?
Rust seems to be a great language and I simply want to take the chance to learn Rust.

## Why not using Github Cli?
Github Cli is an awesome tool but it is still beta. Furthermore it cab only fetch repository information on a per repo basis. However, we would like to fetch information about a set of projects and potentially update a set of projects. 

