#!/bin/sh

# This small script helps make installing the lil-scraper easier
# by automating platform and version detection

set -e

help() {
    cat <<'EOF'
Install a binary release of the lil-scraper
Usage:
    install.sh [options]
Options:
    -h, --help      Display this message
    --tag TAG       Tag (version) of the lil-scraper to install (default <latest release>)
    --to LOCATION   Where to install the binary (default /usr/local/bin)
EOF
}

say() {
    echo "lil-scraper install.sh: $1"
}

say_err() {
    say "$1" >&2
}

err() {
    if [ ! -z $td ]; then
        rm -rf $td
    fi

    say_err "ERROR $1"
    exit 1
}

need() {
    if ! command -v $1 > /dev/null 2>&1; then
        err "need $1 (command not found)"
    fi
}

detect_architecture() {
    local _ostype="$(uname -s)"
    local _cputype="$(uname -m)"

    if [ "$_ostype" = Darwin -a "$_cputype" = i386 ]; then
        # Darwin `uname -s` lies
        if sysctl hw.optional.x86_64 | grep -q ': 1'; then
            local _cputype=x86_64
        fi
    fi

    case "$_ostype" in
        Linux)
            local _ostype=unknown-linux-musl
            ;;

        Darwin)
            local _ostype=apple-darwin
            ;;

        MINGW* | MSYS* | CYGWIN*)
            local _ostype=pc-windows-msvc
            ;;

        *)
            err "no precompiled binaries available for OS: $_ostype"
            ;;
    esac

    case "$_cputype" in
        x86_64 | x86-64 | x64 | amd64)
            local _cputype=x86_64
            ;;
        arm64)
            local _cputype=aarch64
            ;;
        *)
            err "no precompiled binaries available for CPU architecture: $_cputype"

    esac

    arch="$_cputype-$_ostype"
}

while test $# -gt 0; do
    case $1 in
        --help | -h)
            help
            exit 0
            ;;
        --tag)
            tag=$2
            shift
            ;;
        --to)
            dest=$2
            shift
            ;;
        *)
            ;;
    esac
    shift
done

# Dependencies
need basename
need curl
need install
need mkdir
need mktemp
need tar

# Optional dependencies
if [ -z $tag ]; then
    need grep
    need sed
fi

if [ -z $tag ]; then
    need rev
fi

git="walterbm/lil-scraper"

url="https://github.com/$git"

say_err "Downloading lil-scraper from: $url"

url="$url/releases"

if [ -z $tag ]; then
    tag=$(curl --silent "https://api.github.com/repos/$git/releases/latest" |  grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    say_err "Tag: latest ($tag)"
else
    say_err "Tag: $tag"
fi

detect_architecture

say_err "OS Architecture: $arch"

if [ -z $dest ]; then
    dest="/usr/local/bin"
fi

say_err "Installing to: $dest"

url="$url/download/$tag/lil-scraper-$tag-$arch.tar.gz"

td=$(mktemp -d || mktemp -d -t tmp)

curl -sL $url | tar -C $td -xz

test -x $td/lil-scraper-$tag-$arch/lil-scraper

mkdir -p $dest

install -m 755 $td/lil-scraper-$tag-$arch/lil-scraper $dest

rm -rf $td

say_err "Installation Complete! lil-scraper is ready!"