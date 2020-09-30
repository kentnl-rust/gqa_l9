if [[ -z "$1" ]]; then
  echo "pass a package to update dummy"
  exit 1
fi
if [[ -e vendor/Cargo.lock ]]; then
  mv -v vendor/Cargo.lock ./Cargo.lock || exit 1
fi
if [[ -e vendor/config ]]; then
  rm -v vendor/config || exit 1
fi
cargo update -v --dry-run || exit 1
cargo update -v -p "$1" || exit 1
cargo vendor --versioned-dirs || exit 1
cp -v Cargo.lock vendor/ || exit 1
cp -v Cargo.toml vendor/Cargo.toml.orig || exit  1
git rev-parse HEAD > vendor/master-commit || exit 1
cat <<"EOF" > vendor/config
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF
