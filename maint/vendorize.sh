if [[ -e vendor/Cargo.lock ]]; then
  rm -v vendor/Cargo.lock || exit 1
fi
if [[ -e vendor/config ]]; then
  rm -v vendor/config || exit 1
fi
cargo update || exit 1
cargo vendor --versioned-dirs || exit 1
cp -v Cargo.lock vendor/ || exit 1
cat <<"EOF" > vendor/config
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF
