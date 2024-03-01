current_version=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
versions=$(curl -s https://crates.io/api/v1/crates/RuStream | jq -r '.versions | map(.num)')
latest_version=$(echo "$versions" | jq -r '.[0]')
echo "Current Package Version: ${current_version}"
echo "Latest Package Version: $latest_version"
version_exists=false
for version in $(echo "$versions" | jq -r '.[]'); do
    if [ "$version" == "$current_version" ]; then
        version_exists=true
        break
    fi
done
if [ "$version_exists" = true ]; then
  echo "Current version ['$current_version'] found in crates.io"
  release="false";
else
  echo "Current version ['$current_version'] unavailable in crates.io"
  release="true";
fi

if [ "$release" == "true" ]; then
  echo "Creating PROD release"
  release_tag="v$current_version"
  cargo_prerelease=("alpha" "beta" "rc")
  prerelease=false
  for cargo_pre in "${cargo_prerelease[@]}"; do
    if [[ "$current_version" == *"$cargo_pre"* ]]; then
      prerelease=true
      break
    fi
  done
else
  echo "Creating TEST release"
  epoch="$(date +%s)"
  version_as_int=$((10#${current_version//./}))
  ((version_as_int++))
  major=$((version_as_int / 100))
  minor=$((version_as_int % 100 / 10))
  patch=$((version_as_int % 10))
  new_version="$major.$minor.$patch"
  echo "Bumped version to: $new_version"
  release_tag="v${new_version}-prerelease-${epoch}"
  prerelease=true
fi
#commit_msg="$(git log -1 --pretty=%B | sed ':a;N;$!ba;s/\n/\\n/g')"
#commit_msg=$(git log -1 --pretty=%B | tr '\n' '\\n' | awk '{$1=$1};1')
commit_msg=$(git log -1 --pretty=%B | tr -d '\n' | sed 's/^[ \t]*//;s/[ \t]*$//' | sed 's/$/\\n/')

release_data="{\"tag_name\":\"$release_tag\",\"name\":\"$release_tag\",\"body\":\"$commit_msg\",\"draft\":false,\"prerelease\":$prerelease}"
echo ""
echo "$release_data"
