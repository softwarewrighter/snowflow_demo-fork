use std::cell::RefCell;
use std::rc::Rc;
use xetal_value::{Env, Slot, Value, lookup};

pub(crate) type Captured = Rc<RefCell<Env<'static>>>;
pub(crate) type Keeper = &'static mut dyn FnMut(&Value<'static>);

pub(crate) fn keeper(captured: Captured) -> Keeper {
    Box::leak(Box::new(move |value: &Value<'static>| {
        if let Value::Closure(closure) = value {
            *captured.borrow_mut() = closure.env.clone();
        }
    }))
}

pub(crate) fn mesh(captured: &Captured) -> Result<Value<'static>, String> {
    let env = captured.borrow();
    let slot = lookup(&env, "frame!").or_else(|| lookup(&env, "mesh!")).ok_or("frame capture is not ready")?;
    match &*slot.borrow() {
        Slot::Value(value) => Ok(value.clone()),
        _ => Err("mesh binding is not an evaluated value".into()),
    }
}
