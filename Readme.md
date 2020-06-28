# up

## WIP - unusable at the moment

- features
    - default opendns resolver


# Todo
Bold are important

- *add customizable cloudflare/google dns resolvers*
- generate dhparams and use them
- *logging*
- decide fallback index.html or 404
- print help message when no sub command is provided
- location should be configurable instead of just /
- add too many tests
- client max body size configurable?
- *Multiple sites at once*


# Notes
- root is not required when reverse proxy is enabled
- #[cfg(unix)] for main function? won't be supporting windows anyhow
- assumed that `/etc/nginx` is going to be default path, and other folder structure of nginx is unchanged