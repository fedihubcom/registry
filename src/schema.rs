table! {
    employee_infos (id) {
        id -> Int4,
        employee_id -> Int4,
        locale -> Varchar,
        name -> Varchar,
    }
}

table! {
    employees (id) {
        id -> Int4,
        name -> Varchar,
        role -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        encrypted_password -> Varchar,
    }
}

joinable!(employee_infos -> employees (employee_id));

allow_tables_to_appear_in_same_query!(
    employee_infos,
    employees,
    users,
);
