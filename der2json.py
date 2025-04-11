import asn1tools
import json
import sys

s2i = asn1tools.compile_files("./str2int.asn")

encoded = sys.stdin.buffer.read()
decoded = s2i.decode(
	"StrToIntMap",
	encoded,
)

mapd = map(json.dumps, decoded)
prints = map(print, mapd)
sum(1 for _ in prints)
