use crate::helpers::ctx::SuperAdmin;
use crate::routes::RspcResult;
use prisma_client::prisma;

#[serde_zod::codegen]
#[derive(rspc::Type, serde::Deserialize, Debug)]
pub struct AddSchoolRequest {
	pub idempotence_token: String,
	pub name: String,
}

pub async fn add_school(ctx: SuperAdmin, req: AddSchoolRequest) -> RspcResult<prisma::school::Data> {
	ctx.db
		.school()
		.create(req.name, vec![])
		.exec()
		.await
		.map_err(std::convert::Into::into)
}
