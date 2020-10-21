-- Your SQL goes here

CREATE TABLE employees (
  id SERIAL PRIMARY KEY,
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

CREATE UNIQUE INDEX index_employee_infos_on_employee_id_and_locale
  ON employee_infos USING btree (employee_id, locale);
