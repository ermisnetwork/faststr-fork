use sqlx::{database::HasValueRef, Database, Decode, Encode};

impl<'r, DB: Database> Decode<'r, DB> for crate::FastStr where String: Decode<'r, DB>{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let value = <String as Decode<DB>>::decode(value)?;
        Ok(crate::FastStr::from(value))
    }
}

impl<DB: Database> Encode<DB> for crate::FastStr where String: Encode<DB>{
    fn encode_by_ref(&self) -> <DB as HasValueRef<'_>>::ValueRef {
        self.as_str().encode_by_ref()
    }
}