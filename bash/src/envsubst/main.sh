#!/bin/bash -eux

TEXT='World!' envsubst < hello.json

# =>
# {
#   "Hello": "World!"
# }
