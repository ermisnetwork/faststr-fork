use sqlx::{database::HasValueRef, Database, Decode};
// impl<R> FromRow<'_, R> for crate::FastStr where R: Row {
//     fn from_row(row: &R) -> sqlx::Result<Self> {
//         let s: String = FromRow::from_row(row)?;
//     }
// }

impl<'r, DB: Database> Decode<'r, DB> for crate::FastStr where String: Decode<'r, DB>{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let value = <String as Decode<DB>>::decode(value)?;
        Ok(crate::FastStr::from(value))
    }
}