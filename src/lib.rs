use std::time::Instant;

/// 基于 RAII 的耗时统计器。
///
/// 离开作用域时自动触发 `Drop` 打印耗时。
///
/// # Examples
///
/// ```
/// use time_span::TimeSpan;
/// {
///     let _guard = TimeSpan::new("heavy_work");
///     // 耗时操作...
/// } // _guard 离开作用域，自动打印耗时
/// ```
pub struct TimeSpan {
    name: String,
    start: Instant,
}

impl TimeSpan {
    /// 创建并启动一个新的计时器。
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
        }
    }
}

impl Drop for TimeSpan {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        log::info!(
            "[TIME] {} took {:.3} ms",
            self.name,
            elapsed.as_secs_f64() * 1000.0
        );
    }
}

/// 提供简化的宏封装，避免手动声明未使用变量。
///
/// # Examples
///
/// ```
/// use time_span::time_span;
/// fn process() {
///     time_span!("process_func");
///     // 耗时操作...
/// }
/// ```
#[macro_export]
macro_rules! time_span {
    ($name:expr) => {
        let _time_span_guard = $crate::TimeSpan::new($name);
    };
}

/// 执行闭包并统计耗时，返回闭包的结果。
///
/// # Examples
///
/// ```
/// use time_span::measure_time;
/// let result = measure_time("calc_sum", || {
///     1 + 1
/// });
/// ```
pub fn measure_time<T, F>(name: &str, mut f: F) -> T
where
    F: FnMut() -> T,
{
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    log::info!(
        "[TIME] {} took {:.3} ms",
        name,
        elapsed.as_secs_f64() * 1000.0
    );
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_time_span_new() {
        let _span = TimeSpan::new("test_span");
    }

    #[test]
    fn test_time_span_drop() {
        {
            let _span = TimeSpan::new("drop_test");
        }
    }

    #[test]
    fn test_measure_time_basic() {
        let result = measure_time("basic_test", || 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_measure_time_with_sleep() {
        let result = measure_time("sleep_test", || {
            thread::sleep(Duration::from_millis(10));
            "done"
        });
        assert_eq!(result, "done");
    }

    #[test]
    fn test_measure_time_with_return() {
        let result = measure_time("return_test", || {
            let x = 10;
            let y = 20;
            x + y
        });
        assert_eq!(result, 30);
    }

    #[test]
    fn test_macro_basic() {
        time_span!("macro_test");
    }

    #[test]
    fn test_macro_in_function() {
        fn test_func() {
            time_span!("func_test");
            assert!(true);
        }
        test_func();
    }

    #[test]
    fn test_multiple_spans() {
        {
            let _span1 = TimeSpan::new("span1");
            {
                let _span2 = TimeSpan::new("span2");
            }
        }
    }

    #[test]
    fn test_nested_measure_time() {
        let result = measure_time("outer", || measure_time("inner", || 100));
        assert_eq!(result, 100);
    }

    #[test]
    fn test_measure_time_with_mut() {
        let mut count = 0;
        let result = measure_time("mut_test", || {
            count += 1;
            count
        });
        assert_eq!(result, 1);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_time_span_with_string() {
        let name = String::from("dynamic_string_test");
        let _span = TimeSpan::new(name);
    }

    #[test]
    fn test_time_span_with_format() {
        let id = 42;
        let name = format!("task_{}", id);
        let _span = TimeSpan::new(name);
    }

    #[test]
    fn test_time_span_with_str_ref() {
        let s = "string_reference";
        let _span = TimeSpan::new(s);
    }

    #[test]
    fn test_measure_time_with_string() {
        let name = String::from("measure_with_string");
        let result = measure_time(&name, || "test_result");
        assert_eq!(result, "test_result");
    }

    #[test]
    fn test_measure_time_with_format() {
        let id = 123;
        let name = format!("measure_task_{}", id);
        let result = measure_time(&name, || 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_macro_with_dynamic_name() {
        let name = String::from("macro_dynamic");
        time_span!(name.as_str());
    }
}
