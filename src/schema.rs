table! {
    donation_crypto_addresses (id) {
        id -> Int4,
        name -> Varchar,
        code -> Varchar,
        address -> Varchar,
        history -> Varchar,
    }
}

table! {
    employee_contacts (id) {
        id -> Int4,
        employee_id -> Int4,
        name -> Varchar,
        link -> Varchar,
    }
}

table! {
    employee_infos (id) {
        id -> Int4,
        employee_id -> Int4,
        locale -> Varchar,
        name -> Varchar,
        role -> Varchar,
    }
}

table! {
    employees (id) {
        id -> Int4,
        image -> Varchar,
        name -> Varchar,
        role -> Varchar,
    }
}

table! {
    reports (id) {
        id -> Int4,
        datetime -> Timestamp,
        party -> Varchar,
        amount -> Varchar,
        download -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        encrypted_password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    donation_crypto_addresses,
    employee_contacts,
    employee_infos,
    employees,
    reports,
    users,
);
