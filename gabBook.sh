#!/bin/bash

ownership_page="ch04-00-understanding-ownership.html"

../mdBook/target/debug/mdbook build --dest-dir ../docs ./rustBook
firefox docs/${ownership_page} || open docs/${ownership_page}
