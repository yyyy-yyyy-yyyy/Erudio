mod public;
mod user;

use backend_prisma_client::{prisma::PrismaClient, User};
use backend_session_manager::load_session;
use redis::aio::MultiplexedConnection;
use rspc::{Config, ErrorCode};
use std::{path::PathBuf, sync::Arc};
use tower_cookies::Cookies;

pub type RspcResult<T> = Result<T, rspc::Error>;
const SESSION_COOKIE_NAME: &str = "SessionId";

#[derive(Clone)]
pub struct Ctx {
	pub(crate) db: Arc<PrismaClient>,
	pub(crate) redis: MultiplexedConnection,
	pub(crate) cookies: Cookies,
}

#[derive(Clone)]
#[allow(dead_code)] // TODO
pub struct AuthCtx {
	db: Arc<PrismaClient>,
	redis: MultiplexedConnection,
	user: User,
}

pub(crate) fn router() -> rspc::Router<Ctx> {
	rspc::Router::<Ctx>::new()
		.config(
			Config::new()
				// Doing this will automatically export the bindings when the `build` function is called.
				.export_ts_bindings(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../bindings.ts")),
		)
		.merge("public.", public::mount())
		.middleware(|mw| {
			mw.middleware(|mw| async move {
				let old_ctx: Ctx = mw.ctx.clone();
				match old_ctx.cookies.get(SESSION_COOKIE_NAME) {
					Some(session_id) => {
						match load_session(old_ctx.db.clone(), &mut old_ctx.redis.clone(), session_id.value(), None)
							.await?
						{
							Some(user) => Ok(mw.with_ctx(AuthCtx {
								db: old_ctx.db,
								redis: old_ctx.redis,
								user,
							})),
							None => Err(rspc::Error::new(ErrorCode::Unauthorized, "Unauthorized".into())),
						}
					}
					None => Err(rspc::Error::new(ErrorCode::Unauthorized, "Unauthorized".into())),
				}
			})
		})
		.merge("user.", user::mount())
		.build()
}

#[cfg(test)]
mod tests {
	use crate::router;

	#[test]
	fn test_rspc_router() {
		router();
	}
}
