#[test]
fn should_transform_struct_to_flow() {
    let data = r#"
    type Region struct {
        Country string `json:"country" binding:"required"`
    }
    "#;
    let expected_data = "export type Region = { country:string; };".to_string();
    let result = js_typify_gostruct::transform_to_flow(data.to_string()).unwrap();
    assert_eq!(result, expected_data)
}
#[test]
#[should_panic]
fn should_return_errors_if_struct_is_faulty() {
    let data = r#"
    type Region struct
        Country string `json:"country" binding:"required"`
    }
    "#;
    js_typify_gostruct::transform_to_flow(data.to_string()).unwrap();
}
