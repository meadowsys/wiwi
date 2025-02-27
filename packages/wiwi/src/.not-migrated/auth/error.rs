pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error {
	inner: ErrorInner
}

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
	#[error(transparent)]
	P384ECC(#[from] p384::elliptic_curve::Error),
	#[error(transparent)]
	P384ECDSA(#[from] p384::ecdsa::Error),
	#[error(transparent)]
	P384PKCS8DER(#[from] p384::pkcs8::der::Error),
}

impl<T: Into<ErrorInner>> From<T> for Error {
	#[inline]
	fn from(inner: T) -> Self {
		Self { inner: inner.into() }
	}
}
