# Changesets

This directory contains changeset files that track changes for versioning and changelog generation.

## Creating a Changeset

Every PR must include a changeset file:

```bash
# Manual approach:
cat > .changeset/my-feature.md << 'EOF'
---
"@reverbdotcom/cli": minor
---

Brief description of the change.
EOF
```

Use `patch` for fixes/chores, `minor` for new features, `major` for breaking changes.
