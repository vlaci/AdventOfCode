# SPDX-FileCopyrightText: 2025 László Vaskó <opensource@vlaci.email>
#
# SPDX-License-Identifier: EUPL-1.2

(import spork/test)
(import /main :as impl)

(def test-input
  ``
  L68
  L30
  R48
  L5
  R60
  L55
  L1
  L99
  R14
  L82
  ``)

(test/start-suite 0)

(def commands (impl/parse test-input))

(test/assert (compare commands
		      [
		       "L" 68
		       "L" 30
		       "R" 48
		       "L" 5
		       "R" 60
		       "L" 55
		       "L" 1
		       "L" 99
		       "R" 14
		       "L" 82
		       ]))

(test/assert (= (impl/part-1 commands) 3))

(test/assert (= (impl/part-2 commands) 6))

(test/end-suite)
