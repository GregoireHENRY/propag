RELEASE=$1
NAME="propag"
OUTDIR="./"
FLAGS=""

[[ "$RELEASE" == "release" ]] && BUILD="release" && FLAGS+="--release " || BUILD="debug"

cargo build $FLAGS

cp target/$BUILD/$NAME $OUTDIR
strip $OUTDIR$NAME
