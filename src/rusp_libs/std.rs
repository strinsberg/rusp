pub const RUSP_LIB_STD: &str = r#";; Derived Expressions ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(macro-rules let []
  [(let [(var val) ...] body body* ...)
   ((lambda [var ...] body body* ...) val ...)])

(macro-rules let* []
  [(let* [] body body* ...)
   (let [] body body* ...)]
  [(let* [binding binding* ...] body body* ...)
   (let [binding] (let* [binding* ...] body body* ...))])

(macro-rules and []
  [(and) #t]
  [(and test) test]
  [(and test test* ...)
   (if test (and test* ...) #f)])

(macro-rules or []
  [(or) #f]
  [(or test) test]
  [(or test test* ...)
   (let [(!!-or-x-!! test)]
     (if !!-or-x-!! !!-or-x-!! (or test* ...)))])

;; Booleans ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(def not (lambda [x] (if x #f #t)))


;; Collections ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(def first (lambda [xs] (nth 0 xs)))
(def second (lambda [xs] (nth 1 xs)))
(def third (lambda [xs] (nth 2 xs)))
(def fourth (lambda [xs] (nth 3 xs)))
(def fifth (lambda [xs] (nth 4 xs)))

(def empty? (lambda [xs] (= 0 (length xs))))


;; Math/Numbers ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(def exact? (lambda [x] (and (number? x) (not (float? x)))))
(def zero? (lambda [x] (and (number? x) (= 0 x))))
(def positive? (lambda [x] (and (number? x) (> x 0))))
(def negative? (lambda [x] (and (number? x) (< x 0))))
(def non-negative? (lambda [x] (or (zero? x) (positive? x))))

"#;