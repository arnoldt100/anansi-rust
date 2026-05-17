#! /usr/bin/env bash

cd sphinx_root
uv run sphinx-quickstart --sep -p Ansnsi -a 'Arnold Tharrington' -v 0.1.0 --use-make-mode --makefile --batchfile
