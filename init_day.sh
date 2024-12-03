#!/usr/bin/env zsh

set -o errexit
set -o nounset
set -o pipefail

day=$1
day_package="day${day}"

impl=$day_package/src/main.rs

input="inputs/${day}.in" 
example="inputs/${day}.example" 

url="https://adventofcode.com/${AOCYEAR:-2024}/day/${day}"

if [[ ! -d $day_package ]]; then
    cargo new $day_package --bin

    cat << EOF >> $day_package/Cargo.toml
aoc_derive.path = '../aoc_derive'
utils.path = '../utils'
derive_more.workspace = true
itertools.workspace = true
lazy-regex.workspace = true
parse-display.workspace = true
rayon.workspace = true
regex.workspace = true
num.workspace = true

[dev-dependencies]
pretty_assertions.workspace = true
EOF

    cat << EOF > $impl
use aoc_derive::aoc_main;
use utils::ParseInput;
use utils::*;
use lazy_regex::regex;

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"
                "#,
            ""
        );
    }
}
EOF

    git add $day_package

    session=$(cat .session)
    mkdir -p inputs
    curl "$url/input" -H "Cookie: session=$session" > $input
    touch $example
fi


i3-msg "workspace 1; exec firefox $url"
sleep 0.1
i3-msg "workspace 2"

nvim -c "lua require'aoc'.init($day, '$impl', '$input')"
