run_quote() {
  printf "\e[32;1m*\e[0m "
  printf "%q " "$@"
  printf "\n"
  "$@"
}

# Copy state from vendor/ dir so it can be used for
# cargo update
restore_config() {
  if [[ -e vendor/Cargo.lock ]]; then
    cp -v vendor/Cargo.lock ./Cargo.lock || exit 1
  fi
}

# Clean state from vendor/ dir to avoid
# weirdness with cargo vendor even being possible
clean_backup_config() {
  if [[ -e vendor/Cargo.lock ]]; then
    rm -v vendor/Cargo.lock || exit 1
  fi
  if [[ -e vendor/config ]]; then
    rm -v vendor/config || exit 1
  fi
}

# No-changes, just check if Cargo.lock *could* change
update_check() {
  run_quote cargo outdated || exit 1
}

# Construct state in vendor/ as a reference point
backup_config() {
  cp -v Cargo.lock vendor/ || exit 1
  cp -v Cargo.toml vendor/Cargo.toml.orig || exit  1
  git rev-parse HEAD > vendor/master-commit || exit 1
  cat <<"EOF" > vendor/config
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF
}

# Build/clean/update vendor/  as per current Cargo.lock
vendorize() {
  run_quote cargo vendor -v -Z minimal-versions -Z no-index-update --versioned-dirs || exit 1
}

# Sync ./ to vendor/
# update one package
# update vendor/
# sync vendor/ to ./
oneshot_update() {
  local package="$1"

  restore_config || exit 1
  clean_backup_config || exit 1
  update_check || exit 1
  run_quote cargo update -v -Z no-index-update -p "${package}" || exit 1
  vendorize || exit 1
  backup_config || exit 1
  exit 0
}

# Do no work, just check for updates to vendor/
check_updates() {
  restore_config || exit 1
  update_check || exit 1
  exit 1
}

if [[ -z "$1" ]]; then
  echo "Package to update omitted, doing update-check only"
  check_updates
  exit 1
fi

echo "Updating \"$1\""
oneshot_update "$1" || exit 1
exit 0
