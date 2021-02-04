VERSION=`cargo metadata --no-deps --format-version=1 | jq '.packages[0].version' | sed 's/"//g'`
git tag v$VERSION
git push origin v$VERSION