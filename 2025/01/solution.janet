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

(defn main [&]
  (def input (string/trim (file/read stdin :all)))
  (def commands (peg/match grammar input))
  (pp commands)
  (print (part-1 commands)))

