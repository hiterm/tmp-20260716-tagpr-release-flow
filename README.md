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

Verification has not run yet.

