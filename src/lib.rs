use std::cell::Cell;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy)]
struct Context {
    trace_id: u128,
    span_id: u64,
}

thread_local! {
    static CONTEXT: Cell<Context> = const { Cell::new(Context {
        trace_id: 0,
        span_id: 0,
    }) };
}

struct Span {
    trace_id: u128,
    span_id: u64,
    parent_span_id: u64,
    start_time: Option<SystemTime>,
    end_time: Option<SystemTime>,
}

impl Span {
    fn new(trace_id: u128, span_id: u64, parent_span_id: u64) -> Self {
        Self {
            trace_id,
            span_id,
            parent_span_id,
            start_time: None,
            end_time: None,
        }
    }

    fn enter(mut self) -> SpanGuard {
        self.start();
        let saved_context = CONTEXT.replace(Context {
            trace_id: self.trace_id,
            span_id: self.span_id,
        });
        SpanGuard {
            saved_context,
            span: self,
        }
    }

    fn start(&mut self) {
        self.start_time = Some(SystemTime::now());
    }

    fn end(&mut self) {
        self.end_time = Some(SystemTime::now());
    }
}

struct SpanGuard {
    saved_context: Context,
    span: Span,
}

impl Drop for SpanGuard {
    fn drop(&mut self) {
        self.span.end();
        let cur_context = CONTEXT.get();
        if cur_context.trace_id == self.span.trace_id && cur_context.span_id == self.span.span_id {
            CONTEXT.set(self.saved_context);
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
