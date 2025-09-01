use crate::Ulid;
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::sqlite::{SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef};
use sqlx::{Decode, Encode, Sqlite, Type};

impl Type<Sqlite> for Ulid {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for Ulid {
    fn encode_by_ref(
        &self,
        args: &mut Vec<SqliteArgumentValue<'q>>,
    ) -> Result<IsNull, BoxDynError> {
        let s = self.to_string();
        <String as Encode<'q, Sqlite>>::encode_by_ref(&s, args)
    }
}

impl<'r> Decode<'r, Sqlite> for Ulid {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let s = <String as Decode<Sqlite>>::decode(value)?;
        Ulid::from_string(&s).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::Ulid;

    #[test]
    fn sqlx_sqlite_roundtrip() {
        let ulid = Ulid::from_string("3Q38XWW0Q98GMAD3NHWZM2PZWZ").unwrap();
        let s = ulid.to_string();
        let decoded_ulid = Ulid::from_string(&s).unwrap();
        assert_eq!(ulid, decoded_ulid);
    }
}
