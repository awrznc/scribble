target_string = 'piyopipi'
regex_pattern = '^(piyo)(pi..)$'

re = Regexp.compile(regex_pattern)
p re.match(target_string)
# => #<MatchData "piyopipi" 1:"piyo" 2:"pipi">
