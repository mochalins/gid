# gid

> Git ID Manager

`gid` is a Git identity manager that helps switch between multiple "profiles"
of Git configurations. `gid` is non-intrusive, injecting configuration options
on a per-command basis rather than editing local or global Git configurations.
Furthermore, `gid` is a cross-platform utility that only requires an available
`git` command in the environment as a dependency.

## Usage

`gid` itself is a wrapper around the `git` command and can be used as a drop-in
replacement. All options and arguments passed to `gid` will be forwarded to the
`git` command with the active Git profile's configuration settings prepended.

```console
gid commit -m "This is a commit"
```

An accompanying utility `gidc` is used to select and manage Git profiles.

> **Warning**
> Before `gid` or its accompanying utility `gidc` are used, a configuration
> file must be created

### Configuration

`gid` usage requires an existing configuration file in [TOML](https://toml.io)
syntax.

This file must be named `gid.toml`, and can be placed in one of two locations
(listed in order of priority):

1. Any path pointed to by an environment variable `GID_CONFIG`
2. In the current working directory where the `gid` or `gidc` commands are used

An example `gid.toml`:

```toml
active = "profile_name_1"  # The current active Git profile must be provided

[profile.profile_name_1]  # Profile names come after a mandatory `profile.`
user.name = "my_git_username"
user.email = "my_git_email@example.com"
user.signingkey = "my_git_signingkey"
commit.gpgsign = true
tag.gpgsign = true
pull.rebase = false
sshkey = "$HOME/.ssh/my_ssh_key"

[profile.whatever_other_name]  # Profiles can specify as few or many
                               # configuration options as desired
user.name = "my_git_username_2"
user.email = "i-only-want-to-change-these-settings@example.com"
sshkey = "$HOME/.ssh/id_rsa"
```

### `gidc`

The `gidc` utility can be used to manage Git profiles in the `gid`
configuration file. All commands and options can be found in the `--help`
section of the `gidc` utility.

```console
gidc --help
```

#### Select an active profile

An active Git profile can be selected with the `set` command.

```console
gidc set <profile_name>
```
> **Warning**
> The provided profile name must be a valid name in the configuration file

#### List all profiles

All available profile names in the configuration file can be listed with the
`list` command.

```console
gidc list
```

#### Export profile to Git configuration

Any defined profile can be exported to either a repository-local or the global
Git configuration with the `export` command.

```console
gidc export [-g] [profile_name]
```

The `-g` flag indicates whether the profile should be exported to the global
Git configuration. If not set, `export` will work with the local Git
configuration by default.

A profile name can be provided to `export` to specify which profile should be
used. If not provided, the current active profile will be used by default.
