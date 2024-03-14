//! A debounced function; or a function that won't actually get called until
//! there haven't been calls to it for a certain amount of time.

super::runtime_selection_compile_check!("debounce");

#[cfg(feature = "tokio")]
pub use tokio::*;

#[cfg(feature = "tokio")]
mod tokio {
	use super::dyn_fn;
	use ::chrono::{ Local, NaiveDateTime, TimeDelta };
	use ::std::{ mem::swap, sync::Arc };
	use ::tokio::runtime::Handle;
	use ::tokio::sync::Mutex;
	use ::tokio::sync::mpsc::{ UnboundedReceiver, UnboundedSender, unbounded_channel };
	use ::tokio::time::sleep;

	/// Returns a new function that debounces calls to the passed function. This
	/// function can be cloned however many times you want and passed across threads.
	///
	/// If the feature `debounce-dyn-fn` is enabled, this function (and the other 3 exposed) will
	/// wrap the function into a `Box<dyn Fn>`, to use dynamic dispatch and avoid
	/// monomorphisation binary size cost.
	///
	/// If you would like to call the provided function on the leading edge,
	/// [`debounce_immediate`] is what you're after.
	///
	/// Note: requires async runtime to run, and must be called in the context of
	/// an async runtime. The returned function however, does not have to be called
	/// in an async runtime context. If you would like to initialise a debounced
	/// function in a non-async context, and have a runtime handle that will outlive
	/// the debounced function, [`debounce_with_rt`] and [`debounce_immediate_with_rt`]
	/// may be interesting.
	///
	/// The reason a runtime is needed: debounce spawns two background tasks on
	/// the runtime. One of them receives messages from the calls, and stores the
	/// time in which the last call occured. The second one is the one that owns the
	/// passed function, and is the one reading the times the first task stores,
	/// sleeping for the right amount of time, then calling the function. These
	/// background tasks will exit themself after the last debounced handle is dropped.
	///
	/// If the background tasks get stopped (caused by ex. the backing runtime is
	/// stopped), then the returned debounce function will panic.
	///
	/// I suppose there could be two threads spawned in the background, but that
	/// felt like an overkill thing heh, perhaps there could be a static runtime
	/// available here, enabled with a feature.
	#[inline]
	pub fn debounce(
		f: impl Fn() + Send + 'static,
		wait_in_ms: usize
	) -> impl Fn() + Clone + Send + Sync + 'static {
		_debounce(dyn_fn(f), wait_in_ms, false, &current_rt())
	}

	/// Returns a new function that debounces calls to the passed function. This
	/// returns one that calls the function on the leading edge of the delay. See
	/// [`debounce`] for more information.
	#[inline]
	pub fn debounce_immediate(
		f: impl Fn() + Send + 'static,
		wait_in_ms: usize
	) -> impl Fn() + Clone + Send + Sync + 'static {
		_debounce(dyn_fn(f), wait_in_ms, true, &current_rt())
	}

	/// Returns a new function that debounces calls to the passed function, using
	/// the provided runtime handle to spawn the background tasks needed to handle
	/// debouncing. See [`debounce`] for more information.
	#[inline]
	pub fn debounce_with_rt(
		f: impl Fn() + Send + 'static,
		wait_in_ms: usize,
		handle: &Handle
	) -> impl Fn() + Clone + Send + Sync + 'static {
		_debounce(dyn_fn(f), wait_in_ms, false, handle)
	}

	/// Returns a new function that debounces calls to the passed function, using
	/// the provided runtime handle to spawn the background tasks needed to handle
	/// debouncing. This returns one that calls the function on the leading edge
	/// of the delay. See [`debounce`] for more information.
	#[inline]
	pub fn debounce_immediate_with_rt(
		f: impl Fn() + Send + 'static,
		wait_in_ms: usize,
		handle: &Handle
	) -> impl Fn() + Clone + Send + Sync + 'static {
		_debounce(dyn_fn(f), wait_in_ms, true, handle)
	}

	struct DebounceInternalArgs<F> {
		f: F,
		immediate: bool,
		last_call_time: Arc<Mutex<Option<NaiveDateTime>>>,
		debounce_time: TimeDelta
	}

	/// setup fn
	fn _debounce(
		f: impl Fn() + Send + 'static,
		wait_in_ms: usize,
		immediate: bool,
		rt_handle: &Handle
	) -> impl Fn() + Clone + Send + Sync + 'static {
		let debounce_time = TimeDelta::try_milliseconds(wait_in_ms as _).unwrap();
		let (sender, receiver) = unbounded_channel();
		let (fn_caller_sender, fn_caller_receiver) = unbounded_channel();
		let last_call_time = Arc::new(Mutex::new(None));

		let args = DebounceInternalArgs { f, immediate, last_call_time, debounce_time };
		rt_handle.spawn(recv_task(receiver, fn_caller_sender, Arc::clone(&args.last_call_time)));
		rt_handle.spawn(fn_caller(fn_caller_receiver, args));

		move || sender.send(()).unwrap()
	}

	/// receives calls from debounce function, sending message to fn_caller if
	/// last call time is None, and continually updates last call time as it
	/// receives messages
	async fn recv_task(
		mut receiver: UnboundedReceiver<()>,
		fn_caller_sender: UnboundedSender<()>,
		last_call_time: Arc<Mutex<Option<NaiveDateTime>>>
	) {
		while receiver.recv().await.is_some() {
			let mut old_time = Some(Local::now().naive_local());

			let mut lock = last_call_time.lock().await;
			swap(&mut *lock, &mut old_time);
			drop(lock);

			// initial state is None
			// fn_caller sets it back to None when a cycle successfully completes
			if old_time.is_none() {
				fn_caller_sender.send(()).unwrap();
			}
		}
	}

	/// receives a message when the next cycle should begin, then continually
	/// pulls last call time to check if it needs to wait more. Sets last call
	/// time to None once its run the function, so recv_task will send a new message
	/// once it receives calls again
	async fn fn_caller<F>(
		mut receiver: UnboundedReceiver<()>,
		args: DebounceInternalArgs<F>
	)
	where
		F: Fn() + Send + 'static
	{
		while receiver.recv().await.is_some() {
			// and the cycle begins anew
			if args.immediate { (args.f)() }

			loop {
				let last_call_time = {
					let lock = args.last_call_time.lock().await;
					let time = *lock;
					drop(lock);
					// message is sent only after this is set to Some by recv_task
					time.unwrap()
				};
				let now = Local::now().naive_local();
				let delta = now - last_call_time;
				let remaining = args.debounce_time - delta;

				if remaining <= TimeDelta::zero() { break }

				sleep(remaining.to_std().unwrap()).await;
			}

			if !args.immediate { (args.f)() }

			let mut lock = args.last_call_time.lock().await;
			*lock = None;
			drop(lock);
		}
	}

	#[inline]
	fn current_rt() -> Handle {
		Handle::try_current()
			.expect("debounce functions can only be created within the context of a tokio runtime")
	}
}

#[cfg(feature = "debounce-dyn-fn")]
#[inline(always)]
fn dyn_fn(
	f: impl Fn() + Send + 'static
) -> Box<dyn Fn() + Send> {
	Box::new(f)
}

#[cfg(not(feature = "debounce-dyn-fn"))]
#[inline(always)]
fn dyn_fn(
	f: impl Fn() + Send + 'static
) -> impl Fn() + Send + 'static {
	f
}
