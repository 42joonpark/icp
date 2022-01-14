use crate::authorize::token::TokenInfo;
use crate::authorize::AccessToken;

struct Program {
	pub token: AccessToken,
	pub me: TokenInfo,
}