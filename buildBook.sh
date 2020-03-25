#!/bin/bash
red=$'\e[1;31m'
end=$'\e[0m'

printf "Running the following examples: \n" > buildBook.log
declare -a targetExamples=(
    "one_var" # book_04_01_01_one_var
    "scope" # book_04_01_02_scope
    "string_from" # book_04_01_03_scope
    "reference" # book_04_01_04_reference
    "scope2" # book_04_01_05_scope2
    "int_copy" # book_04_01_06_int_copy
    "string_move" # book_04_01_07_string_move
    "string_clone" # book_04_01_09_string_clone
    "int_copy_2" # book_04_01_10_int_copy_2
    "ownership_function" # book_04_01_11_ownership_function
    "return_values" # book_04_01_12_return_values
    "return_tuple" # book_04_01_13_return_tuple
    "pass_reference" # book_04_02_01_pass_reference
    "acquire_from_function" # book_04_02_02_acquire_from_function
    "safely_out_of_scope" # book_04_02_03_safely_out_of_scope
    "mut_ref" # book_04_02_05_mut_ref
    "scope_reference" # book_04_02_07_scope_reference
    "shared_and_unique_borrow" # book_04_02_09_shared_and_unique_borrow
    "no_dangle" # book_04_02_12_no_dangle
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
