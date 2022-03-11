#!/usr/bin/env bash
set -e
cd "`dirname $0`"
. ./config.sh

if [$1 == '--dev']; then
    near dev-deploy --wasmFile ../out/nft_pawnshop.wasm --accountId $PAWNSHOP_ID
else
    near deploy --wasmFile ../out/nft_pawnshop.wasm --accountId $PAWNSHOP_ID
fi

near call $PAWNSHOP_ID new --account_id $PAWNSHOP_ID