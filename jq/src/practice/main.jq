def pingpong:
    .
;

def increment(target):
    target + 1
;

def if_example:
    if .number == 0 then
        1
    elif .number == 256 then
        2
    else
        3
    end
;

# entry point
.
# =>
# {
#   "color": {
#     "red": 255,
#     "green": 255,
#     "blue": 255
#   }
# }

| [ . ]
# =>
# [
#   {
#     "color": {
#       "red": 255,
#       "green": 255,
#       "blue": 255
#     }
#   }
# ]

| .[] | .color
# =>
# {
#   "red": 255,
#   "green": 255,
#   "blue": 255
# }

| pingpong | { "number": increment(.red) }
# =>
# {
#   "number": 256
# }

| if_example
# => 2

| . as $num | $num
# => 2

| [ $num, $num, $num] | map(. + 1)
# =>
# [
#   3,
#   3,
#   3
# ]

| to_entries
# =>
# [
#   {
#     "key": 0,
#     "value": 3
#   },
#   {
#     "key": 1,
#     "value": 3
#   },
#   {
#     "key": 2,
#     "value": 3
#   }
# ]

| map(.value + .key)
# =>
# [
#   3,
#   4,
#   5
# ]

| @csv
# => "3,4,5"
