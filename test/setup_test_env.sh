hab install core/libarchive
hab install core/libsodium
hab install core/openssl
export LIBARCHIVE_LIB_DIR=$(hab pkg path core/libarchive)/lib
export LIBARCHIVE_INCLUDE_DIR=$(hab pkg path core/libarchive)/include
export LIBARCHIVE_STATIC=true
export SODIUM_LIB_DIR=$(hab pkg path core/libsodium)/lib
export SODIUM_STATIC=true
export OPENSSL_LIB_DIR=$(hab pkg path core/openssl)/lib
export OPENSSL_INCLUDE_DIR=$(hab pkg path core/openssl)/include
export OPENSSL_STATIC=true
