# Releasing

## Tagging

1) Ensure CI is green.
2) Update `CHANGELOG.md` with the release version/date.
3) Create an annotated tag:

```bash
git tag -a v1.0.0 -m "v1.0.0: initial stable CLI bot release"
git push origin v1.0.0
```

## GitHub Release

- Title: `v1.0.0`
- Notes: summarize major features and breaking changes
- (Optional) Attach binaries if you later add a release build workflow
