

use sqlx::{database::{HasArguments, HasValueRef}, encode::IsNull, postgres::PgArgumentBuffer, sqlite::SqliteArgumentValue, Any, Database, Decode, Encode, MySql, Postgres, Sqlite, Type};

use crate::FastStr;

impl<'r, DB: Database> Decode<'r, DB> for crate::FastStr where String: Decode<'r, DB>{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let value = <String as Decode<DB>>::decode(value)?;
        Ok(crate::FastStr::from(value))
    }
}

impl<'q> Encode<'q, Any> for crate::FastStr {
    fn encode_by_ref(&self, args: &mut <Any as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        <String as Encode<Any>>::encode_by_ref(&self.to_string(), args)
    }
}

impl<'q> Encode<'q, Sqlite> for crate::FastStr {
    fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue<'q>>) -> IsNull {
        <String as Encode<Sqlite>>::encode_by_ref(&self.to_string(), args)
    }
}


impl Encode<'_, Postgres> for FastStr {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        <&str as Encode<Postgres>>::encode(&**self, buf)
    }
}

impl Encode<'_, MySql> for FastStr {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> IsNull {
        <&str as Encode<MySql>>::encode(&**self, buf)
    }
}


impl<DB: Database> Type<DB> for crate::FastStr where String: Type<DB> {
    fn type_info() -> <DB as Database>::TypeInfo {
        <String as Type<DB>>::type_info()
    }
}