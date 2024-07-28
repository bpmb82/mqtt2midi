#!/bin/bash

if [ $# -eq 0 ]
  then
    echo "No arguments supplied"
    echo "Usage:./create_release.sh <tag>"
    exit 1
fi

NEW_TAG=$1

if [[ $(git status -s) ]]; then
  echo "You have uncommitted changes, please commit or stash them before creating a release."
  exit 1
fi

echo "Creating release $NEW_TAG"
git tag -a $NEW_TAG -m "Release $NEW_TAG"

echo "Creating changelog for release $NEW_TAG"
git log --pretty="* %s" "$(git --no-pager tag --sort=creatordate --merged $NEW_TAG | tail -2 | head -1)..HEAD" --no-merges > CHANGELOG.md

echo "Pushing changes to origin"
git add .
git commit -m "Add changelog for release $NEW_TAG"
git push

echo "Pushing tag $NEW_TAG to origin"
git push origin $NEW_TAG

echo "Done"