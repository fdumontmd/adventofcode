cat input.txt | egrep "(..).*\1"  | egrep "(.).\1"| wc
