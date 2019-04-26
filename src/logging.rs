macro_rules! trace_checkbox {
    ($predicate:expr, $title:expr) => {
        {
            let checkbox = if $predicate { "[x]" } else { "[ ]" };
            trace!("{} - {}", checkbox, $title);
        }
    }
}
