# tagpr release flow verification

Temporary public fixture for verifying a `tagpr`-based release flow before
adopting it in `hiterm/bookshelf-api`.

The verification covers:

- synchronized Rust workspace, lockfile, and Render image versions;
- patch and minor release selection;
- GitHub Release creation;
- direct, same-workflow GHCR publication using the `tagpr` tag output;
- building a Render-style Dockerfile from the exact released image tag;
- observation of whether a release created with `GITHUB_TOKEN` starts a
  separate `release: published` workflow.

No workflow publishes a `latest` image tag.

## Results

Verified successfully on 2026-07-16.

| Scenario | Result | Evidence |
| --- | --- | --- |
| Default patch bump | `2.8.1` to `2.8.2` | [Release PR #1](https://github.com/hiterm/tmp-20260716-tagpr-release-flow/pull/1), [release run](https://github.com/hiterm/tmp-20260716-tagpr-release-flow/actions/runs/29432303437) |
| `bump-minor` label | `2.8.2` to `2.9.0` | [Release PR #7](https://github.com/hiterm/tmp-20260716-tagpr-release-flow/pull/7), [release run](https://github.com/hiterm/tmp-20260716-tagpr-release-flow/actions/runs/29432647681) |
| Release PR validation | Passed with `cargo check --workspace --locked`, `cargo test --workspace --locked`, and the exact-image check | [dispatched CI](https://github.com/hiterm/tmp-20260716-tagpr-release-flow/actions/runs/29432594058) |
| Exact GHCR image | Push and subsequent `rendercom/Dockerfile` build passed for `2.8.2` and `2.9.0` | [2.9.0 release](https://github.com/hiterm/tmp-20260716-tagpr-release-flow/releases/tag/2.9.0) |
| Separate release workflow | Did not start for a release created with `GITHUB_TOKEN` | `release-observer.yml` has no runs |

The version changes in `Cargo.lock` were limited to the two workspace package
entries. No dependency versions changed. The `2.8.2` and `2.9.0` tags point
directly at their corresponding release-PR merge commits.

## Findings

- Use the `tagpr` action's `tag` output to publish and verify the container in
  the same workflow. Do not rely on a separate `release: published` workflow
  when `tagpr` uses `GITHUB_TOKEN`.
- A release PR branch updated by `GITHUB_TOKEN` needs explicit CI dispatch.
  This fixture uses `workflow_dispatch`, reports the result as a commit status,
  and retries dispatch to tolerate short Git ref propagation delays.
- `postVersionCommand = cargo check --workspace` keeps `Cargo.lock` synchronized
  after `tagpr` updates the workspace manifests.
- Listing GHCR package versions through the REST API was not tested because the
  local GitHub CLI token lacks `read:packages`. The release workflows verified
  publication by pulling the newly pushed tag in the Render-style build.
