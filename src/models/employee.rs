use crate::database::DbConn;

use crate::schema::employees;
use crate::schema::employee_contacts;
use crate::schema::employee_infos;

use diesel::prelude::*;

#[derive(Debug, Identifiable, Serialize, Queryable)]
pub struct Employee {
    pub id: i32,
    pub image: String,
    pub name: String,
    pub role: String,
}

#[derive(Associations, Debug, Identifiable, Serialize, Queryable)]
#[belongs_to(Employee)]
pub struct EmployeeContact {
    pub id: i32,
    pub employee_id: i32,
    pub name: String,
    pub link: String,
}

#[derive(Associations, Debug, Identifiable, Serialize, Queryable)]
#[belongs_to(Employee)]
pub struct EmployeeInfo {
    pub id: i32,
    pub employee_id: i32,
    pub name: String,
    pub role: String,
}

impl Employee {
    pub fn all(db_conn: DbConn) -> Result<Vec<Self>, diesel::result::Error> {
        employees::table.load::<Self>(&*db_conn)
    }

    pub fn all_with_contacts(db_conn: DbConn) -> Result<Vec<(Self, Vec<EmployeeContact>)>, diesel::result::Error> {
        let employees = employees::table.load::<Self>(&*db_conn)?;
        let contacts = EmployeeContact::belonging_to(&employees).load::<EmployeeContact>(&*db_conn)?.grouped_by(&employees);

        Ok(employees.into_iter().zip(contacts).collect::<Vec<_>>())
    }
}
