use crate::models::Result;
use crate::schema::nouns;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(primary_key(n_sg))]
pub struct Noun {
    pub gender: String,
    pub declension: String,
    pub n_sg: String,
    pub g_sg: String,
    pub d_sg: String,
    pub acc_sg: String,
    pub ab_sg: String,
    pub voc_sg: String,
    pub n_pl: String,
    pub g_pl: String,
    pub d_pl: String,
    pub acc_pl: String,
    pub ab_pl: String,
    pub voc_pl: String,
    pub translation: String,
    pub def: String,
}

pub fn add_noun(conn: &mut MysqlConnection, noun: Noun) -> Result<()> {
    diesel::insert_into(nouns::table)
        .values(noun)
        .execute(conn)?;
    Ok(())
}
