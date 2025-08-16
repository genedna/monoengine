pub mod errors;
pub mod config;

/// MonoError 类型别名
/// 
/// 注意：这里的类型定义存在问题，应该是 Result<T, MonoError> 而不是 Result<(), MonoError>
/// 建议修改为更通用的形式
pub type MonoResult<T> = Result<T, errors::MonoError>;