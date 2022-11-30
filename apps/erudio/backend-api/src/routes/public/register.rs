use crate::{
	helpers::consts::{SALT_SIZE, SECRET_SIZE},
	routes::{RspcResult, SESSION_COOKIE_NAME},
	Ctx,
};
use argon2::{Config, ThreadMode, Variant, Version};
use cookie::SameSite;
use error_handler::InternalError;
use log::debug;
use prisma_client::prisma::{pii_data, user, GrammaticalForm};
use rand::RngCore;
use rspc::Type;
use session_manager::init_session;
use tower_cookies::Cookie;

#[derive(Type, serde::Deserialize, Debug)]
pub struct RegisterRequest {
	pub idempotence_token: String,
	pub email: String,
	pub password: String,
	pub code: (),
}

pub(crate) async fn register(ctx: Ctx, req: RegisterRequest) -> RspcResult<()> {
	debug!("Register Request : {:?}", req);
	let argon_config: Config = Config {
		variant: Variant::Argon2i,
		version: Version::Version13,
		mem_cost: 16384,
		time_cost: 3,
		lanes: 4,
		thread_mode: ThreadMode::Parallel,
		secret: ctx.argon_secret.as_slice(),
		ad: &[],
		hash_length: 32,
	};
	let mut salt = vec![0; SALT_SIZE];
	let mut connection_secret = vec![0; SECRET_SIZE];
	{
		let mut rng = rand::thread_rng();
		rng.fill_bytes(&mut salt);
		rng.fill_bytes(&mut connection_secret);
	}
	// TODO transaction + if email duplicate then correct error
	let user = ctx
		.db
		.user()
		.create(
			argon2::hash_encoded(req.password.as_bytes(), &salt, &argon_config).map_err(Into::<InternalError>::into)?,
			vec![],
		)
		.exec()
		.await?;

	let pii_data = ctx
		.db
		.pii_data()
		.create(GrammaticalForm::Indeterminate, user::id::equals(user.id.clone()), vec![
			pii_data::email::Set(Some(req.email)).into(),
		])
		.exec()
		.await?;

	let user = user::Data {
		pii_data: Some(Some(Box::new(pii_data))),
		..user
	};

	ctx.cookies.add(
		Cookie::build(
			SESSION_COOKIE_NAME,
			init_session(&ctx.db, &mut ctx.redis.clone(), &user, &connection_secret, None).await?,
		)
		.secure(false) // TODO change one we have ssl set up
		.http_only(true)
		.same_site(SameSite::Strict)
		.finish(),
	);

	Ok(())
}
