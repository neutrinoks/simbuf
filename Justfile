# Little helpers:
# - to hide the command itself in the output, place '@' symbol before the command
#   or list available tasks by defining it as: `just --list`
# - for defaults in case of executing just `just` define: `default: taskx tasky`

# Lists available tasks
default:
    just --list

# Run the latest build
test:
    # Check linter and formatting
    cargo +nightly fmt --check
    cargo clippy --locked --no-deps -- -D warnings
    # std
    cargo t -r
    # std parity-scale-codec
    cargo t -r --all-features
    # -
    cargo t -r --no-default-features
    # parity-scale-codec
    cargo t -r --no-default-features --features parity-scale-codec
