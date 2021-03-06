#ifndef GW_DEF_H_
#define GW_DEF_H_

#include "stddef.h"

#define GW_KEY_BYTES 32
#define GW_VALUE_BYTES 32

/* Key type */
#define GW_ACCOUNT_KV 0
#define GW_ACCOUNT_NONCE 1
#define GW_ACCOUNT_PUBKEY_HASH 2
#define GW_ACCOUNT_CODE_HASH 3

#define GW_MAX_RETURN_DATA_SIZE 1024
/* 128KB */
#define GW_MAX_ARGS_SIZE 131072

/* Call receipt */
typedef struct {
  uint8_t return_data[GW_MAX_RETURN_DATA_SIZE];
  uint32_t return_data_len;
} gw_call_receipt_t;

/* layer2 syscalls */

/**
 * Create a new account
 *
 * @param ctx        The godwoken context
 * @param script     Contract's script (MUST be valid molecule format CKB
 * Script)
 * @param script_len Length of script structure
 * @param account_id ID of new account
 * @return           The status code, 0 is success
 */
typedef int (*gw_create_fn)(void *ctx, uint8_t *script, uint32_t script_len,
                            uint32_t *account_id);

/**
 * Load value by key from current contract account
 *
 * @param ctx    The godwoken context
 * @param account_id  account to modify
 * @param key    The key (32 bytes)
 * @param value  The pointer to save the value of the key (32 bytes)
 * @return       The status code, 0 is success
 */
typedef int (*gw_load_fn)(void *ctx, uint32_t account_id,
                          const uint8_t key[GW_KEY_BYTES],
                          uint8_t value[GW_VALUE_BYTES]);
/**
 * Load the nonce of account
 *
 * @param ctx         The godwoken context
 * @param account_id  The account to load nonce
 * @param value       The pointer to save the nonce value of the key (32 bytes)
 * @return            The status code, 0 is success
 */
typedef int (*gw_load_nonce_fn)(void *ctx, uint32_t account_id, uint8_t value[GW_VALUE_BYTES]);

/**
 * Store key,value pair to current account's storage
 *
 * @param ctx    The godwoken context
 * @param account_id  account to read
 * @param key    The key
 * @param value  The value
 * @return       The status code, 0 is success
 */
typedef int (*gw_store_fn)(void *ctx, uint32_t account_id,
                           const uint8_t key[GW_KEY_BYTES],
                           const uint8_t value[GW_VALUE_BYTES]);

/**
 * Set the return data of current layer 2 contract (program) execution
 *
 * @param data   The data to return
 * @param len    The length of return data
 * @return       The status code, 0 is success
 */
typedef int (*gw_set_program_return_data_fn)(void *ctx, uint8_t *data,
                                             uint32_t len);

/**
 * Get account id by account script_hash
 *
 * @param ctx        The godwoken context
 * @param script_hashThe account script_hash
 * @param account_id The pointer of the account id to save the result
 * @return           The status code, 0 is success
 */
typedef int (*gw_get_account_id_by_script_hash_fn)(void *ctx,
                                                   uint8_t script_hash[32],
                                                   uint32_t *account_id);

/**
 * Get account script_hash by account id
 *
 * @param ctx        The godwoken context
 * @param account_id The account id
 * @param script_hashThe pointer of the account script hash to save the result
 * @return           The status code, 0 is success
 */
typedef int (*gw_get_script_hash_by_account_id_fn)(void *ctx,
                                                   uint32_t account_id,
                                                   uint8_t script_hash[32]);

/**
 * Get account's nonce
 *
 * @param ctx        The godwoken context
 * @param account_id The account id
 * @param nonce      The point of the nonce to save the result
 * @return           The status code, 0 is success
 */
typedef int (*gw_get_account_nonce_fn)(void *ctx, uint32_t account_id,
                                       uint32_t *nonce);

/**
 * Get account script by account id
 */
typedef int (*gw_get_account_script_fn)(void *ctx, uint32_t account_id,
                                      uint32_t *len, uint32_t offset,
                                      uint8_t *script);
/**
 * Load data by data hash
 *
 * @param ctx        The godwoken context
 * @param data_hash  The data hash (hash = ckb_blake2b(data))
 * @param len        The length of the script data
 * @param offset     The offset of the script data
 * @param data       The pointer of the data to save the result
 * @return           The status code, 0 is success
 */
typedef int (*gw_load_data_fn)(void *ctx, uint8_t data_hash[32],
                              uint32_t *len, uint32_t offset,
                              uint8_t *data);

typedef int (*gw_store_data_fn)(void *ctx,
                                uint32_t data_len,
                                uint8_t *data);

/**
 * Get layer 2 block hash by number
 *
 * @param ctx        The godwoken context
 * @param block_hash The pointer of the layer 2 block hash to save the result
 * @param number     The number of the layer 2 block
 * @return           The status code, 0 is success
 */
typedef int (*gw_get_block_hash_fn)(void *ctx, uint64_t number,
                                    uint8_t block_hash[32]);

/**
 * Emit a log (EVM LOG0, LOG1, LOGn in polyjuice)
 *
 * @param ctx            The godwoken context
 * @param account_id     The account to emit log
 * @param data           The log data
 * @param data_length    The length of the log data
 * @return               The status code, 0 is success
 */
typedef int (*gw_log_fn)(void *ctx, uint32_t account_id, uint32_t data_length,
                         const uint8_t *data);



/* Godwoken context */
typedef struct {
  uint32_t from_id;
  uint32_t to_id;
  uint8_t args[GW_MAX_ARGS_SIZE];
  uint32_t args_len;
} gw_transaction_context_t;

typedef struct {
  uint64_t number;
  uint64_t timestamp;
  uint32_t aggregator_id;
} gw_block_info_t;

typedef struct {
  /* verification context */
  gw_transaction_context_t transaction_context;
  gw_block_info_t block_info;
  gw_call_receipt_t receipt;
  /* layer2 syscalls */
  gw_load_fn sys_load;
  gw_load_nonce_fn sys_load_nonce;
  gw_store_fn sys_store;
  gw_set_program_return_data_fn sys_set_program_return_data;
  gw_create_fn sys_create;
  gw_get_account_id_by_script_hash_fn sys_get_account_id_by_script_hash;
  gw_get_script_hash_by_account_id_fn sys_get_script_hash_by_account_id;
  gw_get_account_nonce_fn sys_get_account_nonce;
  gw_get_account_script_fn sys_get_account_script;
  gw_load_data_fn sys_load_data;
  gw_store_data_fn sys_store_data;
  gw_get_block_hash_fn sys_get_block_hash;
  gw_log_fn sys_log;
} gw_context_t;

/**
 * Initialize Godwoken context
 */
int gw_context_init(gw_context_t *ctx);

#endif /* GW_DEF_H_ */
