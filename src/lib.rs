#![crate_name = "js_typify_gostruct"]

use interpreters::{select_interpreter, TargetIntepreter};
use scanner::Input;

mod ast;
mod interpreters;
mod parser;
mod scanner;

/// The source holds the go-lang struct(s) that you want to transform
pub struct Source<S>(S);

impl<I> Source<I>
where
    I: Input + Copy,
{
    /// Returns a new Source which can then be transfomed.
    /// # Arguments
    ///
    /// * `input` - Anything that implements the Input trait. (by default this either a `String` or `&str`)
    ///
    /// # Examples
    /// ```
    /// use js_typify_gostruct;
    /// let input = r#"
    /// type Region struct {
    /// Country string `json:"country" binding:"required"`
    /// State string
    /// }"#;
    /// let source = js_typify_gostruct::Source::new(input);
    ///
    /// ```
    pub fn new(input: I) -> Self {
        Self(input)
    }
    /// Returns the transformed go-lang struct.
    /// # Arguments
    ///
    /// * `target` - Target represents the target interpreter that will be used for the transformation. This can be anything that implements the `TargetIntepreter` Trait. By default this can be a `String` or `&str`
    ///
    ///
    /// * #### N/B: The targets that can be used as of now are, "flow" & "typescript" (more to come...).
    /// # Examples
    /// ```
    /// use js_typify_gostruct;
    /// let input = r#"
    /// type Region struct {
    /// Country string `json:"country" binding:"required"`
    /// State string
    /// }"#;
    /// let source = js_typify_gostruct::Source::new(input);
    /// let result = source.transform_to("flow").expect("The struct should be transformed without an issue");
    /// assert!(result.contains("// @flow"))
    ///
    /// ```
    pub fn transform_to<T>(&self, target: T) -> Result<String, Vec<String>>
    where
        T: TargetIntepreter,
    {
        let tokens = scanner::scan(self.0)?;
        let parsed_result = parser::parse(&tokens)?;
        let interpreter = select_interpreter(target)?;
        interpreter
            .interpret(parsed_result)
            .map_err(|err| err.into())
    }
}
