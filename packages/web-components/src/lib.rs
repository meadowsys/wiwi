#![allow(
	clippy::missing_inline_in_public_items,
	reason = "leptos"
)]

use leptos::prelude::*;
use leptos_mview::mview;

#[component]
pub fn Table<C, CV>(children: C) -> impl IntoView
where
	C: FnOnce() -> CV,
	CV: IntoView + Send
{
	mview! {
		table
			role = "table"
		{
			{ children() }
		}
	}
}

#[component]
pub fn Caption<C, CV>(children: C) -> impl IntoView
where
	C: FnOnce() -> CV,
	CV: IntoView + Send
{
	mview! {
		caption
			role = "caption"
		{
			{ children() }
		}
	}
}
