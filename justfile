set dotenv-load

day := `date +%d`
dayWithout0 := trim_start_match(day, "0")
year := "2022"
file := "src/bin/" + day + ".rs"

# Format, lint, and run the program for today.
run: _output-folder
    rustfmt src/bin/{{day}}.rs
    cargo clippy
    # Hide warning here because we just ran clippy
    RUSTFLAGS=-Awarnings cargo run --release --bin {{day}} | tee "output/{{day}}.log"

# Begin working on todays problem. Downloads input, creates template and opens the problem and code.
begin: _input-folder
    echo "use aoc2022::*;\n\nconst INPUT: &str = include_str!(\"../../input/{{day}}\");\n\nfn main() {\n\n}" >> {{file}}
    curl --silent "https://adventofcode.com/{{year}}/day/{{dayWithout0}}/input" -H "Cookie: session=$AOC_SESSION" > "input/{{day}}"
    code {{file}}
    open "https://adventofcode.com/{{year}}/day/{{dayWithout0}}"

# Submit a solution from the previously run program for the given level (1 or 2).
submit level:
    # Find the solution and pass it to the _submit command.
    just _submit {{level}} $(pcregrep -o1 "^level-{{level}}-solution=(.*)$" output/{{day}}.log)

_submit level solution:
    curl --silent "https://adventofcode.com/{{year}}/day/{{dayWithout0}}/answer" \
        -X "POST" \
        -H "Cookie: session=$AOC_SESSION" \
        --data "level={{level}}&answer={{solution}}" \
        | xmllint --html --xpath "//main" - 2> /dev/null

_output-folder:
    mkdir -p output

_input-folder:
    mkdir -p input
