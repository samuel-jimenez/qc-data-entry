#!/bin/sh
# pre-commit.sh


#stash changes
STASH_NAME="pre-commit-$(date +%s)"
git stash save --quiet --keep-index --include-untracked $STASH_NAME

# format
cargo +nightly fmt --check --quiet
RESULT=$?
cargo +nightly fmt

STASHES=$(git stash list)
if [[ $RESULT -ne 0 ]]; then
  if [[ $STASHES == *"$STASH_NAME"* ]]; then
    git stash pop --quiet
  fi
  echo "Files formatted. Re-add and try again."
  exit 1
fi



# Test prospective commit
./run_tests.sh
RESULT=$?

if [[ $STASHES == *"$STASH_NAME"* ]]; then
  git stash pop --quiet
fi

[ $RESULT -ne 0 ] && exit 1
exit 0
