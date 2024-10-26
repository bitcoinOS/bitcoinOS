#[cfg(feature = "info")]
#[macro_export]
macro_rules! log {
    // 模式匹配和展开
    ($arg:expr) => {
        ic_cdk::print(format!(
            "[info]:time:{},file:{},line:{},message:{}",
            ic_cdk::api::time(),
            std::file!(),
            std::line!(),
            $arg
        ))
    };
}
#[cfg(any(feature = "warn", feature = "error"))]
#[macro_export]
macro_rules! log {
    // 模式匹配和展开
    ($arg:expr) => {};
}

#[macro_export]
macro_rules! log_error {
    // 模式匹配和展开
    ($arg:expr) => {
        ic_cdk::print(format!(
            "[error]:time:{},file:{},line:{},message:{}",
            ic_cdk::api::time(),
            std::file!(),
            std::line!(),
            $arg
        ))
    };
}

#[cfg(feature = "error")]
#[macro_export]
macro_rules! log_warning {
    // 模式匹配和展开
    ($arg:expr) => {};
}

#[cfg(any(feature = "warn", feature = "info"))]
#[macro_export]
macro_rules! log_warning {
    // 模式匹配和展开
    ($arg:expr) => {
        ic_cdk::print(format!(
            "[warn]:time:{},file:{},line:{},message:{}",
            ic_cdk::api::time(),
            std::file!(),
            std::line!(),
            $arg
        ))
    };
}
