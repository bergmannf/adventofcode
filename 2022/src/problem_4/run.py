#!/usr/bin/env python3

import sys
infile = sys.argv[1] if len(sys.argv)>1 else "4.in"
data = open(infile).read().strip()
lines = [x.strip() for x in data.split("\n")]
ans = 0
for line in lines:
    one, two = line.split(",")
    s1, e1 = one.split("-")
    s2, e2 = two.split("-")
    s1, e1, s2, e2 = [int(x) for x in [s1, e1, s2, e2]]
    if s1<=s2 and e2<=e1 or s2<=s1 and e1<=e2:
        ans += 1
print(ans)
