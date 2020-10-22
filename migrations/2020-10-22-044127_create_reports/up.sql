-- Your SQL goes here

CREATE TABLE reports (
  id SERIAL PRIMARY KEY,
  datetime TIMESTAMP NOT NULL,
  party VARCHAR NOT NULL,
  amount VARCHAR NOT NULL,
  download VARCHAR NOT NULL
);

-- Insert data

INSERT INTO reports (datetime, party, amount, download) VALUES
  ('2019-02-01', 'Namecheap, Inc.',  '-$9.06',  '/database_files/namecheap-order-42319180.pdf'),
  ('2019-12-01', 'Namecheap, Inc.',  '-$13.16', '/database_files/namecheap-order-51346327.pdf'),
  ('2020-03-01', 'DigitalOcean LLC', '-$36.00', '/database_files/DigitalOcean Invoice 2020 Mar (4455684-414728758).pdf'),
  ('2020-04-01', 'DigitalOcean LLC', '-$14.76', '/database_files/DigitalOcean Invoice 2020 Apr (4455684-415657724).pdf'),
  ('2020-05-01', 'DigitalOcean LLC', '-$12.00', '/database_files/DigitalOcean Invoice 2020 May (4455684-416077081).pdf'),
  ('2020-06-01', 'DigitalOcean LLC', '-$19.27', '/database_files/DigitalOcean Invoice 2020 Jun (4455684-417055952).pdf'),
  ('2020-07-01', 'DigitalOcean LLC', '-$22.85', '/database_files/DigitalOcean Invoice 2020 Jul (4455684-417777310).pdf'),
  ('2020-08-01', 'DigitalOcean LLC', '-$24.00', '/database_files/DigitalOcean Invoice 2020 Aug (4455684-418606400).pdf'),
  ('2020-09-01', 'DigitalOcean LLC', '-$24.00', '/database_files/DigitalOcean Invoice 2020 Sep (4455684-419361545).pdf');
