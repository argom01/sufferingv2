use crate::errors::AppError;
use crate::models::Result;
use crate::prisma::{noun, PrismaClient};

pub async fn add_noun(conn: &PrismaClient, noun: noun::Data) -> Result<()> {
    conn.noun()
        .create(
            noun::gender::set(noun.gender),
            noun::declension::set(noun.declension),
            noun::n_sg::set(noun.n_sg),
            noun::g_sg::set(noun.g_sg),
            noun::d_sg::set(noun.d_sg),
            noun::acc_sg::set(noun.acc_sg),
            noun::ab_sg::set(noun.ab_sg),
            noun::voc_sg::set(noun.voc_sg),
            noun::n_pl::set(noun.n_pl),
            noun::g_pl::set(noun.g_pl),
            noun::d_pl::set(noun.d_pl),
            noun::acc_pl::set(noun.acc_pl),
            noun::ab_pl::set(noun.ab_pl),
            noun::voc_pl::set(noun.voc_pl),
            noun::translation::set(noun.translation),
            noun::def::set(noun.def),
            vec![],
        )
        .exec()
        .await?;
    Ok(())
}
