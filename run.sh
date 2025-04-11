#!/bin/sh

output=./sample.d/output.asn1.der.dat

mkdir -p "./sample.d"

inputKeys(){
	echo helo
	echo wrld
	echo 2025
}

str2int_wazero(){
	wazero \
		run \
		./rs-asn1-uniq-str2int.wasm
}

echo creating str to int map...
inputKeys |
	str2int_wazero |
	dd \
		if=/dev/stdin \
		of="${output}" \
		status=none

echo showing the created map using asn1tools...
cat "${output}" |
	python3 der2json.py |
	jq -c
