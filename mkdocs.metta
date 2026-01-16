(@doc help-md!
  (@desc "Function prints MarkDown documentation for the input atom. Without parameters prints the full standard library documentation.")
  (@params (
    (@param "Input to get documentation for")))
  (@return "Unit atom"))
(: help-md! (-> Atom (->)))
(= (help-md! $atom)
  (case (get-doc $atom) (
    ((@doc-formal (@item $item) (@kind function) (@type $type) (@desc $descr)
                 (@params $params)
                 (@return (@type $ret-type) (@desc $ret-desc)))
      (let () (println! (format-args "Function {}: {} {}" ($item $type $descr)))
      (let () (println! (format-args "Parameters:" ()))
      (let () (for-each-in-atom $params help-param-md!)
      (let () (println! (format-args "Return: (type {}) {}" ($ret-type $ret-desc)))
      () )))))
    ((@doc-formal (@item $item) (@kind function) (@type $type) (@desc $descr))
      (let () (println! (format-args "Function {} (type {}) {}" ($item $type $descr)))
      () ))
    ((@doc-formal (@item $item) (@kind atom) (@type $type) (@desc $descr))
      (let () (println! (format-args "Atom {}: {} {}" ($item $type $descr)))
      () ))
    ($other (Error $other "Cannot match @doc-formal structure") ))))

(: help-md! (-> (->)))
(= (help-md!) (let () (println! "# MeTTa standard library") (help-md-all!)))

(: help-md-all! (-> (->)))
(= (help-md-all!) (let $top-space (mod-space! top)
    (unify $top-space (@doc $name (@desc $desc) $params $ret)
      (let () (println! (format-args "{}\n\t{}" ($name $desc))) Empty)
      Empty)))
(= (help-md!-all) (let $top-space (mod-space! top)
    (unify $top-space (@doc $name (@desc $desc))
      (let () (println! (format-args "{}\n\t{}" ($name $desc))) Empty)
      Empty)))

(@doc help-param-md!
  (@desc "Function used by function help-md! to output parameters using println!")
  (@params (
    (@param "Parameters list")))
  (@return "Unit atom"))
(: help-param-md! (-> Atom (->)))
(= (help-param-md! $param)
  (let (@param (@type $type) (@desc $desc)) $param
    (println! (format-args "  {} {}" ((type $type) $desc))) ))

!(help-md!)
