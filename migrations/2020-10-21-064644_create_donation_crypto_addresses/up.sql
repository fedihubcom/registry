-- Your SQL goes here

CREATE TABLE donation_crypto_addresses (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  code VARCHAR NOT NULL,
  address VARCHAR NOT NULL,
  history VARCHAR NOT NULL
);

CREATE UNIQUE INDEX index_donation_crypto_addresses_on_name
  ON donation_crypto_addresses USING btree (name);

CREATE UNIQUE INDEX index_donation_crypto_addresses_on_code
  ON donation_crypto_addresses USING btree (code);

-- Insert data

INSERT INTO donation_crypto_addresses (name, code, address, history) VALUES
  ('Bitcoin',
    'BTC',
    '35nA4yNtWUMGuVZCa4y49NwRDmsb6t4VQy',
    'https://chain.so/a/v8mtvta'),
  ('Ethereum',
    'ETH',
    '0x741c815266E6A30114874d55074C1D0FDaA5d3c3',
    'https://etherscan.io/address/0x741c815266E6A30114874d55074C1D0FDaA5d3c3'),
  ('Litecoin',
    'LTC',
    'MAaCD7KEzNYSxkdWDwuWwjiKUoqaiUiJN3',
    'https://chain.so/a/u57ypz0'),
  ('Dash',
    'DASH',
    'XszebNZepbAqnW6HFGtt42QQ3xz7xC8vXb',
    'https://chain.so/a/nle35xo'),
  ('Zcash',
    'ZEC',
    't1YNFM5sdeYT1267fZjR2EmWdNCSAx1qHC1',
    'https://chain.so/a/jaiuip0'),
  ('Dogecoin',
    'DOGE',
    'D9aw49SqyF647uuAgnNz7sX1sYebmA9xRt',
    'https://chain.so/a/3_d3d7m');
