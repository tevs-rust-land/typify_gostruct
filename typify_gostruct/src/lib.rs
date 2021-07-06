#![crate_name = "typify_gostruct"]

use interpreters::ToInterpreter;
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
    /// use typify_gostruct;
    /// let input = r#"
    /// type Region struct {
    /// Country string `json:"country" binding:"required"`
    /// State string
    /// }"#;
    /// let source = typify_gostruct::Source::new(input);
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
    /// use typify_gostruct;
    /// let input = r#"
    /// type Region struct {
    /// Country string `json:"country" binding:"required"`
    /// State string
    /// }"#;
    /// let source = typify_gostruct::Source::new(input);
    /// let result = source.transform_to("flow").expect("The struct should be transformed without an issue");
    /// assert!(result.contains("// @flow"))
    ///
    /// ```
    pub fn transform_to<T>(&self, target: T) -> Result<String, Vec<String>>
    where
        T: ToInterpreter,
    {
        let tokens = scanner::scan(self.0)?;
        let parsed_result = parser::parse(&tokens)?;
        let interpreter = target.convert()?;
        interpreter
            .interpret(parsed_result)
            .map_err(|err| err.into())
    }
}

mod tests {

    #[test]
    fn should_transform_struct_with_flow_interpeter_successfully() {
        let input = r#"
            type Region struct {
            Country string `json:"country" binding:"required"`
            State string
        }"#;
        let source = super::Source::new(input);
        let result = source
            .transform_to("flow")
            .expect("The struct should be transformed without an issue");
        assert!(result.contains("// @flow"));
        assert!(result.contains("country : string"));
        assert!(result.contains("State : string"))
    }

    #[test]
    fn should_transform_struct_with_typescript_interpeter_successfully() {
        let input = r#"
            type Region struct {
            Country string `json:"country" binding:"required"`
            State string
        }"#;
        let source = super::Source::new(input);
        let result = source
            .transform_to("typescript")
            .expect("The struct should be transformed without an issue");
        assert!(result.contains("export interface Region"));
        assert!(result.contains("country : string"));
        assert!(result.contains("State : string"))
    }

    #[test]

    fn should_return_error_if_struct_isnt_valid() {
        let input = r#"
            type Region struct {
            Country string `json:"country" binding:"required"`
            State string
        "#;
        let source = super::Source::new(input);
        let result = source.transform_to("typescript");

        assert!(result.is_err())
    }

    #[test]
    fn should_parse_embedded_structs_correctly() {
        let input = r#"
        type Region struct {
        Country
        State string
    }
    "#;
        let source = super::Source::new(input);
        let result = source.transform_to("typescript");
        assert!(result.is_ok())
    }
}
