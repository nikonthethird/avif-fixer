# AVIF fixer

Very simple tool that reads an AVIF file with metadata issues from `stdin`,
and writes the corrected version to `stdout`. Performs no re-encoding of image data.

## Example

```pwsh
avif-fixer < problem.avif > fixed.avif
```