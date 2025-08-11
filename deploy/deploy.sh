#! /usr/bin/sh

#This Source Code Form is subject to the terms of the Mozilla Public
#License, v. 2.0. If a copy of the MPL was not distributed with this
#file, You can obtain one at https://mozilla.org/MPL/2.0/.

repodir="$1"
webdir="$2"

if [ -z "$repodir" ]; then
	echo "Error: no repo directory provided. Exiting..."
	exit 64
fi

if [ -z "$webdir" ]; then
	echo "Error: no web directory provided. Exiting..."
	exit 64
fi

if [ ! -d "$repodir" ]; then
	echo "Error: repo directory $repodir does not exist. Exiting..."
	exit 64
fi

if [ ! -d "$webdir" ]; then
	echo "Error: no web directory $webdir does not exist. Exiting..."
	exit 64
fi

echo "Change directory"
cd $repodir

echo "Check Out"
fossil pull
fossil update

echo "Create Homepage"
pandoc README.md -f commonmark_x -o "$webdir/index.html"

echo "Git Export"
fossil git export

echo "Automated Testing"
cov="cargo llvm-cov --html"
outdir="$webdir/test-results/"
tmpdir=$(mktemp -d)
gen_cov() {
	echo "Test case: $1"
	mkdir $tmpdir/$1
	$cov --output-dir "$tmpdir/$1/llvm-cov" $2 >"$tmpdir/$1/results.txt" 2>&1
	grep 'test result:' "$tmpdir/$1/results.txt"
}
gen_cov default
gen_cov no-default --no-default-features
rm -rf $outdir
mv $tmpdir $outdir
chgrp -R srv $outdir
chmod -R g+rx $outdir

echo "Documentation"
cargo doc --no-deps
mv target/doc/radnelac "$webdir/doc"