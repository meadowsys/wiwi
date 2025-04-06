use crate::prelude_internal::*;

impl_chain_conversions! { [T, S] std::collections::HashSet<T, S> }
impl_chain_conversions! { [T] std::collections::BTreeSet<T> }
impl_chain_conversions! {
	#[cfg(feature = "hashbrown")]
	[T, S] hashbrown::HashSet<T, S>
}

chain_fns! {
	impl and_mut [T, S] std::collections::HashSet<T, S>;
	impl and_mut [T] std::collections::BTreeSet<T>;
	#[cfg(feature = "hashbrown")]
	impl and_mut [T, S] hashbrown::HashSet<T, S>;

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
contains
difference
drain
entry
extract_if
get
get_or_insert
get_or_insert_with
hasher
insert
intersection
is_disjoint
is_empty
is_subset
is_superset
iter
len
new
remove
replace
reserve
retain
shrink_to
shrink_to_fit
symmetric_difference
take
try_reserve
union
with_capacity
with_capacity_and_hasher
with_hasher
Trait Implementations
BitAnd<&HashSet<T, S>>
BitOr<&HashSet<T, S>>
BitXor<&HashSet<T, S>>
Clone
Debug
Default
Eq
Extend<&'a T>
Extend<T>
From<[T; N]>
FromIterator<T>
IntoIterator
IntoIterator
PartialEq
Sub<&HashSet<T, S>>


btree
Methods
append
clear
contains
difference
entry
extract_if
first
get
get_or_insert
get_or_insert_with
insert
intersection
is_disjoint
is_empty
is_subset
is_superset
iter
last
len
lower_bound
lower_bound_mut
new
new_in
pop_first
pop_last
range
remove
replace
retain
split_off
symmetric_difference
take
union
upper_bound
upper_bound_mut
Trait Implementations
BitAnd<&BTreeSet<T, A>>
BitOr<&BTreeSet<T, A>>
BitXor<&BTreeSet<T, A>>
Clone
Debug
Default
Eq
Extend<&'a T>
Extend<T>
From<[T; N]>
FromIterator<T>
Hash
IntoIterator
IntoIterator
Ord
PartialEq
PartialOrd
Sub<&BTreeSet<T, A>>


hashbrown
Methods
allocation_size
allocator
capacity
clear
contains
difference
drain
entry
extract_if
get
get_or_insert
get_or_insert_with
hasher
insert
insert_unique_unchecked
intersection
is_disjoint
is_empty
is_subset
is_superset
iter
len
new
new_in
par_difference
par_drain
par_eq
par_intersection
par_is_disjoint
par_is_subset
par_is_superset
par_symmetric_difference
par_union
remove
replace
reserve
retain
shrink_to
shrink_to_fit
symmetric_difference
take
try_reserve
union
with_capacity
with_capacity_and_hasher
with_capacity_and_hasher_in
with_capacity_in
with_hasher
with_hasher_in
Trait Implementations
BitAnd<&HashSet<T, S, A>>
BitAndAssign<&HashSet<T, S, A>>
BitOr<&HashSet<T, S, A>>
BitOrAssign<&HashSet<T, S, A>>
BitXor<&HashSet<T, S, A>>
BitXorAssign<&HashSet<T, S, A>>
Clone
Debug
Default
Deserialize<'de>
Eq
Extend<&'a T>
Extend<T>
From<HashMap<T, (), S, A>>
From<[T; N]>
FromIterator<T>
FromParallelIterator<T>
IntoIterator
IntoIterator
IntoParallelIterator
IntoParallelIterator
ParallelExtend<&'a T>
ParallelExtend<T>
PartialEq
Serialize
Sub<&HashSet<T, S, A>>
SubAssign<&HashSet<T, S, A>>
*/
