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

(defn sum-invalid [[s e]]
  (var c 0)
  (loop [id :range-to [s e] :when (invalid? id)]
    (+= c id))
  c)

(defn part-1 [ranges]
  (->> ranges (map sum-invalid) (sum)))


(defn main [cmd & args]
  (if (not= (length args) 1)
    (do
      (printf "Usage: %s <input>" cmd)
      (os/exit 1)))
  (def input (string/trim (slurp (get args 0))))
  (def ranges (peg/match grammar input))
  (print "Part 1 solution is " (part-1 ranges)))
