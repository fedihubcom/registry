-- Your SQL goes here

CREATE TABLE employees (
  id SERIAL PRIMARY KEY,
  image VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  role VARCHAR NOT NULL
);

CREATE TABLE employee_infos (
  id SERIAL PRIMARY KEY,
  employee_id SERIAL NOT NULL,
  locale VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  role VARCHAR NOT NULL,

  CONSTRAINT employee_id_fk FOREIGN KEY (employee_id) REFERENCES employees(id)
);

CREATE TABLE employee_contacts (
  id SERIAL PRIMARY KEY,
  employee_id SERIAL NOT NULL,
  name VARCHAR NOT NULL,
  link VARCHAR NOT NULL,

  CONSTRAINT employee_id_fk FOREIGN KEY (employee_id) REFERENCES employees(id)
);

CREATE UNIQUE INDEX index_employee_infos_on_employee_id_and_locale
  ON employee_infos USING btree (employee_id, locale);

CREATE UNIQUE INDEX index_employee_contacts_on_employee_id
  ON employee_contacts USING btree (employee_id, name);

-- Insert data

WITH employee_ids AS (
  INSERT INTO employees (image, name, role)
    VALUES ('/database_files/kotovalexarian.jpg', 'Alex Kotov', 'Founder')
    RETURNING id
    AS employee_id
)
  INSERT INTO employee_contacts (employee_id, name, link) VALUES
    ((SELECT employee_id FROM employee_ids),
      'Matrix',
      'https://matrix.to/#/@kotovalexarian:fedihub.com'),
    ((SELECT employee_id FROM employee_ids),
      'Twitter',
      'https://twitter.com/kotovalexarian'),
    ((SELECT employee_id FROM employee_ids),
      'Facebook',
      'https://fb.com/kotovalexarian');

WITH employee_ids AS (
  INSERT INTO employees (image, name, role)
    VALUES ('/database_files/xuhcc.png', 'Kirill Goncharov', 'System administrator')
    RETURNING id
    AS employee_id
)
  INSERT INTO employee_contacts (employee_id, name, link) VALUES
    ((SELECT employee_id FROM employee_ids),
      'Matrix',
      'https://matrix.to/#/@xuhcc:matrix.org'),
    ((SELECT employee_id FROM employee_ids),
      'GitHub',
      'https://github.com/xuhcc');
