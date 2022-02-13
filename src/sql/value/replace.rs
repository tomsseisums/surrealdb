use crate::dbs::Executor;
use crate::dbs::Options;
use crate::dbs::Runtime;
use crate::err::Error;
use crate::sql::value::Value;

impl Value {
	pub async fn replace(
		&mut self,
		ctx: &Runtime,
		opt: &Options,
		exe: &Executor<'_>,
		val: &Value,
	) -> Result<(), Error> {
		// Clear all entries
		match val.compute(ctx, opt, exe, Some(self)).await? {
			Value::Object(v) => {
				*self = Value::from(v);
				Ok(())
			}
			_ => Ok(()),
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::dbs::test::mock;
	use crate::sql::test::Parse;

	#[tokio::test]
	async fn replace() {
		let (ctx, opt, exe) = mock();
		let mut val = Value::parse("{ test: { other: null, something: 123 } }");
		let res = Value::parse("{ other: true }");
		let obj = Value::parse("{ other: true }");
		val.replace(&ctx, &opt, &exe, &obj).await.unwrap();
		assert_eq!(res, val);
	}
}
