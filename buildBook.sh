#!/bin/bash
red=$'\e[1;31m'
end=$'\e[0m'

printf "Running the following examples: \n" > buildBook.log
declare -a targetExamples=(
    "acquire_from_function"
    "copy"
    "move"
    "no_dangle"
    "one_var"
    "pass_reference"
    "reference"
    "return_values"
    "scope"
    "scope2"
    "shared_and_unique_borrow"
    )

for target in ${targetExamples[@]}; do
    printf "building %s..." $target
    cargo run --example $target &>> buildBook.log
    if [ $? -eq 0 ]; then
        printf "done.\n"
    else
        printf "${red}failed.${end}\n"
    fi
done

ownership_page="ch04-00-understanding-ownership.html"

mdbook build --dest-dir ../docs ./rustBook
firefox docs/${ownership_page} || open docs/${ownership_page}
