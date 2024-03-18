(ns compile)

;; For now just write everything in here. We want to be able to transform rusp code into a form that is as close to our intended rust code as possible. Then we want some code to convert that into the text that will be a file of rust code.

;; A module will be compiled into a struct

;; All def and require targets will become struct fields. These will be initialized inside of an init function that will also contain any top level forms that are going to be run when the file is loaded. It is important that the def initializations and the free expressions are in the same order as in the file.

;; All defn forms will be compiled into methods of the struct. They will take a vector of val and return a val. These can be in any order.

;; Lambdas and closures can be compiled into member functions as well and instantiated as the argument to a constructor that creates a val with them. Closures will take an extra argument which is a vector of captured values. It is important to know how a function is being used. If it is being called and has not been shadowed then it can just be called. If we are applying a variable that might hold a function we need to use apply. If a method or function are passed in a way that makes them a variable then they must be wrapped inside of a lambda to be applied when necessary. It will be important to maintain an environment so it is easy to tell how a function call or use should be dealt with.

;; It should be possible to think about applying simple transformations, and in cases adding metadata, so that we can do some of the more complicated transformations more easily. However, I want to avoid making extra passes when I could easily just pass some metadata along during the pass and use it to complete a transformation all at once. The splitting up of passes should be to make a pass much simpler or to prepare necessary data or structure for future passes to make them possible. Optimizations that can be made during a pass should be, but if they add a lot of complexity we should save them until the code is organized in a way that makes them easiest. For now optimizations will be low priority as we want to get the overall transformation to a nice low simple form before we worry about that, unless the optimizations need data that will only be available during a particular pass.

;; I also want to consider that some transformation functions might be easiest to keep in maps or something and to just apply them based on the symbol or type we encounter. Rather than having big conditionals and a whole bunch of functions to call from there. The place where this will be the most usefull is at the end when we are transforming small expressions on essentially a line by line basis and creating text. we can just map a function over the list of simple forms and return the string and then join them with newlines to make the file.

;; As much as I want to keep the output agnostic of rust as ling as possible, it will be much easier to deal with reordering things before we start converting them into a big list of statements. All require, def, and free expressions should be grouped before the functions. We could even delimit them with some blocks indicating the init function of the class and the implementation block of the class. This only works though if we are compiling to a language where var names do not have to be in order. In this case every top level def, requrie, or defn will be prefixed with self when used and therefore can appear out of order, for example if a def calls a function to bind a value.

;; Utility ;;
(defn zip
  [col1 col2]
  (->> col2
       (interleave col1)
       (partition 2)))

(defn safe-nth [val n] (if (> (count val) n) (nth val n) nil))

(comment
  (nth [1 2 3] 4)
  ;; *err*  java.lang.IndexOutOfBoundsException user REPL:1:1
  (safe-nth [1 2 3] 0) ; 1
  (safe-nth [1 2 3] 1) ; 2
  (safe-nth [1 2 3] 2) ; 3
  (safe-nth [1 2 3] 3) ; nil
  (safe-nth [1 2 3] 4) ; nil
  (safe-nth 45 4)
  ; *err*  java.lang.IllegalArgumentException: Don't know how to create ISeq from: java.lang.Long user REPL:1:1
  ;
)

;; Special Form Transformers ;;

;; Do I really want these all to be single inline instructions? or do I want to maintain somekind of tree like structure? It is certainly more like a low level representation to have everything be as simple an instruction as it can be. I think maybe it will be better to keep everything nested for blocks etc. until right before code generation. So, some of the expand functions below will need to be adjusted a bit to create forms rather than lists to be spliced. Then a late pass can transform it all to have a more rust or even lower level structure. Probably, the best thing is to get it as low as possible while maintaining that tree structure and then after than the passes can be for specific targets.

;; ORDERING
;; This is not a final ordering, but some things need to be first/last and others it is just convinenent if they are first
;; * make if cond
;; * make functions with recur into loop-recur
;;
;;
;; * write as rust code


;; def, defn, let, if, cond, and, or, not

;; NOTE alot of this is experimentation in code and not necessarily meant as usable code. Most should still be tested.

;; would need to walk the whole tree.
;; There are several kinds of expr we can see
;; 1. atoms - all not collection type expressions, e.g. string, number, boolean, etc.
;; 2. list/function application - These are the majority of expr in lisp
;; 3. list/special form - defn, def, if, cond, and, or, let, do (more?)
;; a pass has to deal with all of these. The question is can we make a generic pass that will walk a whole tree and call functions on the elements. Or do we need a generic pass that is used in every pass function to recurse on the children.
;; it should be a post walk, e.g. we want to process the children and then the form itself with the results. Or at least this is what we want in most cases. The easiest first step would be to convert all ifs to conds. There could easily be an if in the true or false branch so we need to resolve those first
(def special-forms #{'def 'defn 'fn 'if 'cond 'let 'and 'or 'do})
(defn special-form?
  [expr]
  (and (or (list? expr) (seq? expr)) (contains? special-forms (first expr))))
(defn fn-application? [expr] (and (list? expr) (not (special-form? expr))))

(comment
  (special-form? 4)
  (fn-application? 5)
  ;
)

(defn pass-post-walk-bindings
  [bindings walk-recur]
  (let [groups (reduce (fn [acc x]
                         (-> acc
                             (update :params conj (first x))
                             (update :exprs conj (second x))))
                       {:params [] :exprs []}
                       (partition 2 bindings))]
    (into [] (interleave (:params groups) (map walk-recur (:exprs groups))))))

(defn pass-post-walk
  [expr pass-fn]
  (println "--post-walk" expr)
  (let [walk-recur
        (fn [x] (println "--walk-recur" x) (pass-post-walk x pass-fn))]
    (pass-fn
     (cond (special-form? expr)
           (do
             (println "--special" expr)
             (case (first expr)
               ;; do not deal with def/defn having a docstring
               def `(~@(take 2 expr) ~@(pass-post-walk (drop 2 expr) pass-fn))
               defn `(~@(take 3 expr) ~@(pass-post-walk (drop 3 expr) pass-fn))
               fn `(~@(take 2 expr) ~@(pass-post-walk (drop 2 expr) pass-fn))
               let `(~(first expr)
                     ~(pass-post-walk-bindings (second expr) walk-recur)
                     ~@(map walk-recur (drop 2 expr)))
               if (cons 'if (map walk-recur (rest expr)))
               cond (cons 'cond (map walk-recur (rest expr)))
               and (cons 'and (map walk-recur (rest expr)))
               or (cons 'or (map walk-recur (rest expr)))
               do (cons 'do (map walk-recur (rest expr)))
               :else nil))
           (fn-application? expr)
           (do (println "--fn-app" expr)
               `(~(first expr)
                 ~@(map (fn [e] (pass-post-walk e pass-fn)) (rest expr))))
           :else (do (println "--else" expr) expr)))))


(defn expand-atoms
  [expr]
  (cond (string? expr) (list :val-string expr)
        ;; need others
        :else expr))

(defn expand-if
  [expr]
  (if (and (special-form? expr) (= 'if (first expr)))
    (concat (list 'cond (second expr) (nth expr 2))
            (if-let [falsy (safe-nth expr 3)]
              (list :else falsy)
              (list :else nil)))
    expr))

(comment
  (expand-atoms "hello, world!")
  (expand-atoms [1 2 3 4])

  (expand-if 45)
  (expand-if '(if true 1))
  (expand-if '(if true 1 2 0))

  (pass-post-walk "hello, world!" expand-atoms)
  (pass-post-walk '(if true "hello, world!" "goodbye") expand-if)
  (pass-post-walk '(if true (if false "hello, world!" 42) "goodbye") expand-if)
  (pass-post-walk '(let [a 45 b 67] (if true a b)) expand-if)
  ;
)

;; Early transformation of if into cond means that if will not be present in forms further into the transformation process

(defn fn-form? [form] (contains? #{'defn 'fn} (first form)))
(defn tail-expr? [form] (contains? #{'cond 'and 'or 'let 'do} (first form)))



;; Return true if there is a recur in a tail position, needs to recurse on all forms that can have a tail position. Error if a recur is found not in tail position.
(defn has-valid-recur? [form])

;; Return the param list from a function form.
(defn get-fn-params
  [expr]
  (case (first expr)
    defn (if (string? (nth expr 2)) (nth expr 3) (nth expr 2))
    fn (second expr)
    :else (ex-info "Not a function expression" {:expr expr})))

;; Return the body from a function form.
(defn get-fn-body
  [expr]
  (case (first expr)
    defn (if (string? (nth expr 2)) (drop 4 expr) (drop 3 expr))
    fn (drop 2 expr)
    :else (ex-info "Not a function expression" {:expr expr})))


;; Create a binding list from a list params by duplicating each param.
(defn make-loop-bindings
  [params]
  (reduce (fn [x acc]
            (-> acc
                (conj x)
                (conj x)))
          params))

(defn expand-loop-assignments
  [assignments]
  (map (fn [[name expr]] (list :loop-assign name expr)) assignments))

;; This needs to look through all defn and lambda expressions and if they have a recur in tail position we convert the function body to a loop-recur
;; This function really needs to be callable on the children so that lambdas inside the body can be transformed. We need a function that can just walk all the forms we might see in a program and call a function on them or a function that will recurse properly on every form that a particular pass might want to do (which means a walking function dedicated to each pass).
(defn expand-fn-recur
  [form]
  (if (and (fn-form? form) (has-valid-recur? form))
    (list loop (make-loop-bindings (get-fn-params form)) (get-fn-body form))
    form))

;; Expand the actual recur expression
(defn expand-recur
  [expr params]
  (cond (= 'recur (first expr)) (map (fn [[p e]] (list :loop-set p e))
                                     (zip params (rest expr)))
        (tail-expr? (first expr))
        (if (contains? #{'let 'and 'or 'do} (first expr))
          `(~@(drop-last expr) ~@(expand-recur (last expr) params))
          ;; otherwise cond
          `(~@(drop-last 2 expr) ~@(expand-recur (take-last 2 expr) params)))
        :else (ex-info "This expression can't recur" {:expr expr})))

;; Given a loop recur construct we can use a mutable variable that is not available to the user to store all of the bindings. Then we simply convert the loop to a rust infinite loop. The recur becomes a block where we set all the new variable values and calls continue. The rest of the tail positions become returns.
;; What if a child has a loop? Do we recurse on each child during this function as well?
(defn expand-loop-recur
  [form]
  (if (= 'loop (first form))
    `(:loop ~(expand-loop-assignments (second form))
            ~@(drop-last (drop 2 form))
            ~@(expand-recur (last form) (second form)))
    form))


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
