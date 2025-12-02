# SPDX-FileCopyrightText: 2025 László Vaskó <opensource@vlaci.email>
#
# SPDX-License-Identifier: EUPL-1.2

(declare-project
  :name "day-02"
  :dependencies ["judge"])

(declare-executable
  :name "day-02"
  :entry "main.janet"
  :install true)

(def *judge* (string (dyn :tree) "/bin/judge"))

(task "test" [] (shell *judge*))

(task "test-i" [] (shell *judge* "-i"))
