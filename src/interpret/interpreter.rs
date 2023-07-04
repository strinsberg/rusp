use crate::data::ExternalRep;
use crate::interpret::{null_env, Vm};
use crate::io::reader::StringReader;
use crate::rusp_libs::std::RUSP_LIB_STD;

// TODO setup proper stdlib with core procedure environment creation and with
// library procedure/macro rusp files. These files need to end with rusp rather
// than scm and not be named SCM_* anymore.
// TODO errors need to be printed as display and not debug when their to_string
// or display rep is implemented.

pub struct Interpreter {
    ready: bool,
    vm: Vm,
}

impl Interpreter {
    // Create a new interpreter with an empty environment
    pub fn new() -> Interpreter {
        Interpreter {
            ready: false,
            vm: Vm::new(null_env()),
        }
    }

    // Initialize the interpreter.
    pub fn init(mut self) -> Interpreter {
        self.ready = true;
        self.load_std();
        self
    }

    pub fn eval_string(&mut self, text: &str) -> String {
        if !self.ready {
            panic!("not initialized");
        }

        match StringReader::new(text).read_forms() {
            Ok(forms) => match self.vm.eval_forms(&forms) {
                Ok(val) => val.to_external(),
                Err(e) => format!("{:?}", e),
            },
            Err(e) => format!("{:?}", e),
        }
    }

    fn load_std(&mut self) {
        let lib_std_forms = match StringReader::new(RUSP_LIB_STD).read_forms() {
            Ok(forms) => forms,
            Err(e) => panic!("failed to read RUSP_LIB_STD: Err: {:?}", e),
        };

        // Eval scheme standard lib with the base env to add all defines to interpreter env
        match self.vm.eval_forms(&lib_std_forms) {
            Err(e) => panic!("failed to eval RUSP_LIB_STD: Err: {:?}", e),
            _ => (),
        }
    }
}

// Testing ////////////////////////////////////////////////////////////////////

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluating_simple_forms_with_definitions() {
        let mut int = Interpreter::new().init();

        // Basic forms
        assert_eq!(int.eval_string(""), "()".to_string());
        assert_eq!(int.eval_string("1"), "1".to_string());

        // Define and use
        assert_eq!(int.eval_string("(define a 5)"), "()".to_string());
        assert_eq!(int.eval_string("a"), "5".to_string());
        assert_eq!(int.eval_string("'a"), "a".to_string());
        assert_eq!(int.eval_string("(+ a 10)"), "15".to_string());

        // Define and call a lambda
        assert_eq!(
            int.eval_string("(define f (lambda (x) (+ x 1)))"),
            "()".to_string()
        );
        assert_eq!(int.eval_string("(f a)"), "6".to_string());

        // Redefine a
        assert_eq!(int.eval_string("(define a 33)"), "()".to_string());
        assert_eq!(int.eval_string("a"), "33".to_string());

        // Multiple forms at once
        assert_eq!(
            int.eval_string("(define b 6)\n\n(+ b a)\n\n(- a b)\n"),
            "27".to_string()
        );

        // Check that the stdlib was infact loaded. MIGHT BREAK LATER.
        assert_eq!(int.eval_string("(equal? a 33)"), "#t".to_string());
    }
}
*/
