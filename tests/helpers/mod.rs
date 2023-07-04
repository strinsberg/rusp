use rusp::interpret::Interpreter;

pub fn eval_assert(code: &str, expected: &str) {
    let result = Interpreter::new().init().eval_string(code);
    assert!(
        &result == expected,
        "\n*** Failed Eval Assert ***\n----\nEvaluated: {}\n----\nActual:    {}\nExpected:  {}\n----\n",
        code,
        result,
        expected
    );
}
