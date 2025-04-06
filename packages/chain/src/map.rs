use crate::prelude_internal::*;

impl_chain_conversions! { [K, V, S] std::collections::HashMap<K, V, S> }
impl_chain_conversions! {
	#[cfg(feature = "hashbrown")]
	[K, V, S] hashbrown::HashMap<K, V, S>
}

chain_fns! {
	impl and_mut [K, V, S] std::collections::HashMap<K, V, S>;
	#[cfg(feature = "hashbrown")]
	impl and_mut [K, V, S] hashbrown::HashMap<K, V, S>;

	doc "awa"
	fn test_todo_remove_me_lol(inner) {
		let _ = inner;
	}
}



/*
std
Methods
capacity
clear
contains_key
drain
entry
extract_if
get
get_disjoint_mut
get_disjoint_unchecked_mut
get_key_value
get_mut
hasher
insert
into_keys
into_values
is_empty
iter
iter_mut
keys
len
new
remove
remove_entry
reserve
retain
shrink_to
shrink_to_fit
try_insert
try_reserve
values
values_mut
with_capacity
with_capacity_and_hasher
with_hasher
Trait Implementations
Clone
Debug
Default
Eq
Extend<(&'a K, &'a V)>
Extend<(K, V)>
From<[(K, V); N]>
FromIterator<(K, V)>
Index<&Q>
IntoIterator
IntoIterator
IntoIterator
PartialEq
UnwindSafe





hashbrown
Methods
allocation_size
allocator
capacity
clear
contains_key
drain
entry
entry_ref
extract_if
get
get_key_value
get_key_value_mut
get_many_key_value_mut
get_many_key_value_unchecked_mut
get_many_mut
get_many_unchecked_mut
get_mut
hasher
insert
insert_unique_unchecked
into_keys
into_values
is_empty
iter
iter_mut
keys
len
new
new_in
par_drain
par_eq
par_keys
par_values
par_values_mut
raw_entry
raw_entry_mut
remove
remove_entry
reserve
retain
shrink_to
shrink_to_fit
try_insert
try_reserve
values
values_mut
with_capacity
with_capacity_and_hasher
with_capacity_and_hasher_in
with_capacity_in
with_hasher
with_hasher_in
Trait Implementations
Clone
Debug
Default
Deserialize<'de>
Eq
Extend<&'a (K, V)>
Extend<(&'a K, &'a V)>
Extend<(K, V)>
From<HashMap<T, (), S, A>>
From<[(K, V); N]>
FromIterator<(K, V)>
FromParallelIterator<(K, V)>
Index<&Q>
IntoIterator
IntoIterator
IntoIterator
IntoParallelIterator
IntoParallelIterator
IntoParallelIterator
ParallelExtend<(&'a K, &'a V)>
ParallelExtend<(K, V)>
PartialEq
Serialize

*/
