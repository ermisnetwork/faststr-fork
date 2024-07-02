use sqlx::{database::{HasArguments, HasValueRef}, encode::IsNull, Database, Decode, Encode, Type, TypeInfo};

impl<'r, DB: Database> Decode<'r, DB> for crate::FastStr where String: Decode<'r, DB>{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let value = <String as Decode<DB>>::decode(value)?;
        Ok(crate::FastStr::from(value))
    }
}

impl<'q, DB: Database> Encode<'q, DB> for crate::FastStr where String: Encode<'q, DB>{
    fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        <String as Encode<DB>>::encode_by_ref(&self.to_string(), buf)
    }
}

impl<DB: Database> Type<DB> for crate::FastStr where String: Type<DB> {
    fn type_info() -> <DB as Database>::TypeInfo {
        <String as Type<DB>>::type_info()
    }
}