use from_residual_pg::TracedResult;

fn bottom() -> TracedResult<(), &'static str> {
    Err("something went wrong")?;
    TracedResult::from_output(())
}

fn middle() -> TracedResult<(), &'static str> {
    bottom()?;
    TracedResult::from_output(())
}

fn top() -> TracedResult<(), &'static str> {
    middle()?;
    TracedResult::from_output(())
}

fn main() {
    match top().handle() {
        Ok(_) => println!("Success"),
        Err((e, t)) => println!("Error: {}\nTrace: {:?}", e, t),
    }
}
