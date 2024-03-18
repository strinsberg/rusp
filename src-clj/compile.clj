(ns compile)

;; For now just write everything in here. We want to be able to transform rusp code into a form that is as close to our intended rust code as possible. Then we want some code to convert that into the text that will be a file of rust code.

;; A module will be compiled into a struct

;; All def and require targets will become struct fields. These will be initialized inside of an init function that will also contain any top level forms that are going to be run when the file is loaded. It is important that the def initializations and the free expressions are in the same order as in the file.

;; All defn forms will be compiled into methods of the struct. They will take a vector of val and return a val. These can be in any order.

;; Lambdas and closures can be compiled into member functions as well and instantiated as the argument to a constructor that creates a val with them. Closures will take an extra argument which is a vector of captured values. It is important to know how a function is being used. If it is being called and has not been shadowed then it can just be called. If we are applying a variable that might hold a function we need to use apply. If a method or function are passed in a way that makes them a variable then they must be wrapped inside of a lambda to be applied when necessary. It will be important to maintain an environment so it is easy to tell how a function call or use should be dealt with.

;; It should be possible to think about applying simple transformations, and in cases adding metadata, so that we can do some of the more complicated transformations more easily. However, I want to avoid making extra passes when I could easily just pass some metadata along during the pass and use it to complete a transformation all at once. The splitting up of passes should be to make a pass much simpler or to prepare necessary data or structure for future passes to make them possible. Optimizations that can be made during a pass should be, but if they add a lot of complexity we should save them until the code is organized in a way that makes them easiest. For now optimizations will be low priority as we want to get the overall transformation to a nice low simple form before we worry about that, unless the optimizations need data that will only be available during a particular pass.

;; I also want to consider that some transformation functions might be easiest to keep in maps or something and to just apply them based on the symbol or type we encounter. Rather than having big conditionals and a whole bunch of functions to call from there. The place where this will be the most usefull is at the end when we are transforming small expressions on essentially a line by line basis and creating text. we can just map a function over the list of simple forms and return the string and then join them with newlines to make the file.

;; As much as I want to keep the output agnostic of rust as ling as possible, it will be much easier to deal with reordering things before we start converting them into a big list of statements. All require, def, and free expressions should be grouped before the functions. We could even delimit them with some blocks indicating the init function of the class and the implementation block of the class. This only works though if we are compiling to a language where var names do not have to be in order. In this case every top level def, requrie, or defn will be prefixed with self when used and therefore can appear out of order, for example if a def calls a function to bind a value.

;; Special Form Transformers ;;

;; Do I really want these all to be single inline instructions? or do I want to
;; maintain somekind of tree like structure? It is certainly more like a low level
;; representation to have everything be as simple an instruction as it can be.

;; def, defn, let, if, cond, and, or, not

(defn expand-binding-list
  [bindings]
  ;; Error if there are not an even number of forms
  (map (fn [b] (list :bind (first b) (second b))) (partition 2 bindings)))

(defn expand-let
  [_ bindings & body]
  (list :block ~@(expand-binding-list bindings) ~@body :block-end))

(defn expand-cond
  [_ & body]
  ;; Error if not an even number of pairs
  ;; If there is not an :else return nil or error???
  `(:block (:if (:truthy? ~(first body)))
           :block ~(second body)
           :end-block
           ~@(mapcat
              (fn [[cond task]]
                (list (list :elif (list :truthy cond)) :block task :block-end))
              (drop 2 body))
           :cond-else :block-end))

(defn expand-if
  [_ condition & paths]
  (expand-cond (concat (list 'cond condition (first paths))
                       (if-let [falsy (second paths)]
                         (list :else falsy)
                         nil))))


(defn expand-top
  [form]
  ;; An expansion for the top level forms. The major reasons to have it split with
  ;;   another version/pass is so that def and defn won't be allowed and to ensure
  ;;   that the top-level structure is presented properly. We can go through the
  ;;   resulting lines and replace them with expansions in other passes.
  ;;   Of course it is impossible to know how many times we will need to expand
  ;;   forms below this level, so it may be necessary to expand all of them recursively
  ;;   starting with the bottom.
  ;; If it is an atom we need to label it with it's type
  ;; Symbols should be noted somehow as they will need special treatment
  ;;   for example we always need to clone a symbol
  ;; If we get a list we need to expand it based on it's first symbol
  ;; Vectors and Maps will need special forms to construct the correct rusp object
  ;;   but only when they are supposed to be literals. For example, in a function
  ;;   definition we probably want to transform it to something that indicates it
  ;;   is an arg list with symbols or destructuring targets.
  ;; Easiest way is to check the type and then to use the type or the first element
  ;;   to access a map that returns the correct function to expand the element.
  ;; The first pass of expansion will not recurse on a lot of forms.
  ;; Remember to expand forms with a tail expression wrapping them in tail call
  ;;   construction where it is appropriate.
  form)



;; Rust Code Generation ;;
(defn mangle "Convert a symbol name into a valid Rust Identifier." [name] name)
(defn construct
  "Rust code for the constructor of the value type"
  [value]
  ;; Not sure if this will be an actual value or a (:val-some-type value)
  ;; Not sure if we will get symbols and need to just return them instead, or add .clone()
  ;; Check it is a valid value type
  value)

(defn block [] "{")
(defn block-end [] "}")
(defn block-assign [_ name] (str "let " name " = {"))
(defn block-end-semi [] (str "};"))

(defn val-assign
  [_ name value]
  (str "let " (mangle name) " = " (construct value) ";"))

;; What if it is a variable???
(defn truthy? [value] (str "__is_truthy__(" (construct value) ")"))
