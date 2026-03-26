use crate::Err;
use core::error::Error as ErrorTrait;
use axum_0_8::response::{ IntoResponse, Response };

impl<E> IntoResponse for Err<E>
where
	E: ErrorTrait + IntoResponse + Send + Sync + 'static
{
	#[inline]
	fn into_response(self) -> Response {
		self.frame.err.into_response()
	}
}
