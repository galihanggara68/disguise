# Git Automation with Disguise

Simplify repetitive Git workflows and complex command sequences.

## Scripts Configuration

```bash
# Sync with main and prune local branches
disguise add --name "git-sync" \
             --command "git checkout main && git pull origin main && git fetch -p && git branch -vv | grep ': gone]' | awk '{print \$1}' | xargs -r git branch -D" \
             --description "Sync with main and cleanup deleted remote branches" \
             --tags "git,workflow"

# Quick commit and push
disguise add --name "git-save" \
             --command "git add . && git commit -m" \
             --description "Stage all changes and commit" \
             --tags "git,utils"

# Undo last commit but keep changes
disguise add --name "git-undo" \
             --command "git reset --soft HEAD~1" \
             --description "Soft reset the last commit" \
             --tags "git,fix"
```

## Usage Examples

### Syncing your local repo
```bash
disguise run git-sync
```

### Quick commit with a message
```bash
disguise run git-save -- "Refactor storage layer"
```
