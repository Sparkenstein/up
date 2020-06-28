# up

## WIP - unusable at the moment

- features
    - default opendns resolver


# Todo

- add customizable cloudflare/google dns resolvers
- generate dhparams and use them
- logging
- decide fallback index.html or 404
- print help message when no sub command is provided
- location should be configurable instead of just /
- check first nginx exists and /etc/nginx is available
- add too many tests
- check if site already exists


# Notes
- root is not required when reverse proxy is enabled
- #[cfg(unix)] for main function? won't be supporting windows anyhow
- assumed that `/etc/nginx` is going to be default path, and other folder structure of nginx is unchanged