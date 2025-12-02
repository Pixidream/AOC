#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

YEAR="2025"
DAY="01"

CRATE_DIR="$ROOT_DIR/$YEAR/$DAY"
README="$ROOT_DIR/$YEAR/README.md"
BENCH_OUTPUT="$ROOT_DIR/$YEAR/bench-output_${YEAR}_${DAY}.txt"

echo "Root dir     : $ROOT_DIR"
echo "Crate dir    : $CRATE_DIR"
echo "Year README  : $README"
echo "Bench output : $BENCH_OUTPUT"
echo

if ! [[ -d "$CRATE_DIR" ]]; then
  echo "Error: crate directory '$CRATE_DIR' not found."
  exit 1
fi

if ! [[ -f "$README" ]]; then
  echo "Error: README '$README' not found."
  exit 1
fi

cd "$CRATE_DIR"

echo "Running cargo bench in $CRATE_DIR..."
cargo bench > "$BENCH_OUTPUT"

echo "Parsing benchmark results from $BENCH_OUTPUT..."

extract_median() {
  local pattern="$1"
  local line

  line=$(grep -A1 "^$pattern" "$BENCH_OUTPUT" | grep "time:" | head -n 1 || true)

  if [[ -z "$line" ]]; then
    echo "n/a"
    return
  fi

  echo "$line" \
    | awk 'match($0, /\[[^]]+\]/) {
             s = substr($0, RSTART+1, RLENGTH-2);
             n = split(s, a, " ");
             if (n >= 4) {
               print a[3] " " a[4];
             } else {
               print "n/a";
             }
           }'
}

example_time=$(extract_median "solve_on_example_input")
real_time=$(extract_median "solve_on_real_input")
big_time=$(extract_median "solve_on_big_synthetic_input")

echo "example: $example_time"
echo "real   : $real_time"
echo "big    : $big_time"

BENCH_MD=$(cat <<EOF
<!-- bench-start -->
| Bench                          | Input size            | Median time |
|--------------------------------|-----------------------|-------------|
| \`solve_on_example_input\`      | small example         | $example_time |
| \`solve_on_real_input\`         | real puzzle input     | $real_time |
| \`solve_on_big_synthetic_input\` | 1,000,000 rotations   | $big_time |
<!-- bench-end -->
EOF
)

perl -0pi -e 's/<!-- bench-start -->.*<!-- bench-end -->/'"$BENCH_MD"'/s' "$README"

echo
echo "Updated benchmarks section in $README"
