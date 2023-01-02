# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 2022-1-2

* feat(web): added `/api/v1/allowed-list` to return list of known domains that breaks due to hiding referrer.
* feat(ext): release: `v0.1.3` to download the list and skip processing those domains. The list is periodically updated.
* fix(ext): release: `v0.1.4` fixes infinite loop with facebook container (ff)
