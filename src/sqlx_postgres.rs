use crate::Ulid;
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use sqlx::{Decode, Encode, Postgres, Type};

impl Type<Postgres> for Ulid {
    fn type_info() -> PgTypeInfo {
        <sqlx::types::Uuid as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for Ulid {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        let uuid = sqlx::types::Uuid::from_u128(self.0);
        uuid.encode_by_ref(buf)
    }
}

impl Decode<'_, Postgres> for Ulid {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let uuid = <sqlx::types::Uuid as Decode<Postgres>>::decode(value)?;
        Ok(Ulid(uuid.as_u128()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ulid;

    #[test]
    fn sqlx_postgres_roundtrip() {
        let ulid = Ulid::from_string("3Q38XWW0Q98GMAD3NHWZM2PZWZ").unwrap();
        let uuid = sqlx::types::Uuid::from_u128(ulid.0);
        let decoded_ulid = Ulid(uuid.as_u128());
        assert_eq!(ulid, decoded_ulid);
    }
}