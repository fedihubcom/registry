-- This file should undo anything in `up.sql`

DROP INDEX index_donation_crypto_addresses_on_code;
DROP INDEX index_donation_crypto_addresses_on_name;

DROP TABLE donation_crypto_addresses;
