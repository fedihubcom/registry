-- This file should undo anything in `up.sql`

DROP INDEX index_employee_infos_on_employee_id_and_locale;

DROP TABLE employee_infos;
DROP TABLE employees;
