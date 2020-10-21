use crate::database::DbConn;

use crate::schema::donation_crypto_addresses;

use diesel::prelude::*;

#[derive(Debug, Identifiable, Serialize, Queryable)]
#[table_name = "donation_crypto_addresses"]
pub struct DonationCryptoAddress {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub address: String,
    pub history: String,
}

impl DonationCryptoAddress {
    pub fn all(db_conn: DbConn) -> Result<Vec<Self>, diesel::result::Error> {
        donation_crypto_addresses::table.load::<Self>(&*db_conn)
    }
}
