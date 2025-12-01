#!/usr/bin/env janet

(def grammar
  '{:direction (capture (choice "L" "R"))
    :amount (number (some :d))
    :command (* :direction :amount (any :s))
    :main (* (some :command) -1)})

(defn part-1 [commands]
  (var position 50)
  (var password 0)
  (each [direction amount] (partition 2 commands)
    (pp [direction amount])
    (let [direction
      (case direction
        "L" -1
        "R" 1)]
      (+= position (* direction amount)))
    (%= position 100)
    (if (< position 0)
      (+= position 100))
    (if (= position 0)
      (++ password)))
  password)

(defn part-2 [commands]
  (var position 50)
  (var password 0)
  (each [direction amount] (partition 2 commands)
    (pp [direction amount])
    (let [direction
      (case direction
        "L" -1
        "R" 1)]
      (var delta (* direction amount))
      (def new (+ position delta))
      (var increment (math/trunc (/ (math/abs (if (or (> delta 0) (= position 0))
                                                new
                                                (+ (- position 100) delta))) 100)))
      (set position (% new 100))
      (if (< position 0)
        (do
          (+= position 100)))
      (+= password increment)))
  password)

(defn main [&]
  (def input (string/trim (file/read stdin :all)))
  (def commands (peg/match grammar input))
  (pp commands)
  (print (part-1 commands))
  (print (part-2 commands)))
