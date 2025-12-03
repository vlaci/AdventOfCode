#!/usr/bin/env janet
# SPDX-FileCopyrightText: 2025 László Vaskó <opensource@vlaci.email>
#
# SPDX-License-Identifier: EUPL-1.2

(def grammar
  ~{:index (number (some :d))
    :range (cmt (* :index "-" :index) ,tuple)
    :main (split "," :range)})

(defn parse [input]
  (peg/match grammar input))

(defn invalid? [id]
  (def id (string id))
  (def length (length id))
  (and (even? length)
       (=
	 (string/slice id 0 (/ length 2))
	 (string/slice id (/ length 2)))))

(defn invalid-2? [id]
  (def id (string id))
  (def length (length id))
  (var invalid false)
  (loop [end :range-to [1 (div length 2)] :when (= (mod length end) 0)]
    (def parts (partition end id))
    (def patt (get parts 0))
    (if (all |(= patt $) parts)
      (do
	(set invalid true)
	(break))))
  invalid)


(defn sum-invalid [p? [s e]]
  (var c 0)
  (loop [id :range-to [s e] :when (p? id)]
    (+= c id))
  c)

(defn part-1 [ranges]
  (->> ranges (map (partial sum-invalid invalid?)) (sum)))

(defn part-2 [ranges]
  (->> ranges (map (partial sum-invalid invalid-2?)) (sum)))


(defn main [cmd & args]
  (if (not= (length args) 1)
    (do
      (printf "Usage: %s <input>" cmd)
      (os/exit 1)))
  (def input (string/trim (slurp (get args 0))))
  (def ranges (peg/match grammar input))
  (print "Part 1 solution is " (part-1 ranges))
  (print "Part 2 solution is " (part-2 ranges)))
