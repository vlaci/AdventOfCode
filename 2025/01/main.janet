#!/usr/bin/env janet
# SPDX-FileCopyrightText: 2025 László Vaskó <opensource@vlaci.email>
#
# SPDX-License-Identifier: EUPL-1.2


(def grammar
  '{:direction (capture (choice "L" "R"))
    :amount (number (some :d))
    :command (* :direction :amount (any :s))
    :main (* (some :command) -1)})

(defn parse [input]
  (peg/match grammar input))

(defn part-1 [commands]
  (var position 50)
  (var password 0)
  (each [direction amount] (partition 2 commands)
    (let [direction
      (case direction
        "L" -1
        "R" 1)]
      (+= position (* direction amount)))
    (set position (mod position 100))
    (if (= position 0)
      (++ password)))
  password)

(defn part-2 [commands]
  (var position 50)
  (var password 0)
  (each [direction amount] (partition 2 commands)
    (let [direction
      (case direction
        "L" -1
        "R" 1)]
      (var delta (* direction amount))
      (def new (+ position delta))
      (def increment (div (math/abs (if (or (> delta 0) (= position 0))
                                      new
                                      (- new 100)))
			  100))
      (set position (mod new 100))
      (+= password increment)))
  password)

(defn main [cmd & args]
  (if (not= (length args) 1)
    (do
      (printf "Usage: %s <input>" cmd)
      (os/exit 1)))
  (def input (string/trim (slurp (get args 0))))
  (def commands (peg/match grammar input))
  (print "Part 1 solution is " (part-1 commands))
  (print "Part 2 solution is " (part-2 commands)))
