# SPDX-FileCopyrightText: 2025 László Vaskó <opensource@vlaci.email>
#
# SPDX-License-Identifier: EUPL-1.2

(import /main :as impl)
(use judge)

(def test-input "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124")

(def ranges (impl/parse test-input))

(test ranges
  @[[11 22]
    [95 115]
    [998 1012]
    [1188511880 1188511890]
    [222220 222224]
    [1698522 1698528]
    [446443 446449]
    [38593856 38593862]
    [565653 565659]
    [824824821 824824827]
    [2121212118 2121212124]])

(test (impl/invalid? 1) false)
(test (impl/invalid? 10) false)
(test (impl/invalid? 11) true)
(test (impl/invalid? 1010) true)
(test (impl/invalid? 1188511885) true)
(test (impl/invalid? 1188911885) false)

(test (impl/part-1 ranges) 1227775554)
