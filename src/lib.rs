use classicube_sys::{
  Chat_Add, Commands_Register, IGameComponent, OwnedChatCommand, OwnedString, Server,
};
use rand::{thread_rng, Rng};
use std::{cell::RefCell, convert::TryInto, mem, os::raw::c_int, ptr, slice};

macro_rules! default_max {
  () => {
    100
  };
}

const ROLL_COMMAND_HELP: &str = concat!(
  "&a/client roll [min] [max] &e(Default 1 to ",
  default_max!(),
  ")"
);

thread_local! {
  static COMMAND: RefCell<OwnedChatCommand> = RefCell::new(OwnedChatCommand::new(
    "Roll",
    c_command_callback,
    false,
    vec![ROLL_COMMAND_HELP],
  ));
}

unsafe extern "C" fn c_command_callback(args: *const classicube_sys::String, args_count: c_int) {
  let args = slice::from_raw_parts(args, args_count.try_into().unwrap());
  let args: Vec<String> = args.iter().map(|cc_string| cc_string.to_string()).collect();

  command_callback(args);
}

macro_rules! check_err {
  ($x:expr) => {
    match $x {
      Ok(v) => v,
      Err(e) => {
        chat_add(format!("Error: &c{}", e));
        return;
      }
    }
  };
}

fn command_callback(args: Vec<String>) {
  let args: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

  let (mut min, mut max): (i64, i64) = match args.as_slice() {
    [min, max] => (check_err!(min.parse()), check_err!(max.parse())),

    [max] => (1, check_err!(max.parse())),

    _ => (1, default_max!()),
  };

  if min > max {
    mem::swap(&mut min, &mut max);
  }

  let mut rng = thread_rng();
  let result = rng.gen_range(min, max + 1);

  chat_add(format!("&f(&e{}&f|&e{}&f) = &a{}", min, max, result));
}

fn chat_add(text: String) {
  let owned_string = OwnedString::new(text);

  unsafe {
    Chat_Add(owned_string.as_cc_string());
  }
}

extern "C" fn init() {
  COMMAND.with(|owned_command| unsafe {
    Commands_Register(&mut owned_command.borrow_mut().command);
  });
}

#[no_mangle]
pub static Plugin_ApiVersion: c_int = 1;

#[no_mangle]
pub static mut Plugin_Component: IGameComponent = IGameComponent {
  /* Called when the game is being loaded. */
  Init: Some(init),
  /* Called when the component is being freed. (e.g. due to game being closed) */
  Free: None,
  /* Called to reset the component's state. (e.g. reconnecting to server) */
  Reset: None,
  /* Called to update the component's state when the user begins loading a new map. */
  OnNewMap: None,
  /* Called to update the component's state when the user has finished loading a new map. */
  OnNewMapLoaded: None,
  /* Next component in linked list of components. */
  next: ptr::null_mut(),
};
