# rust fuzzy search

This is a project I'm using to learn rust. It's messy and incomplete. Don't hate.

How it works
---

It constructs a [Levenshtein automaton](https://en.wikipedia.org/wiki/Levenshtein_automaton)
and uses it to test all the words in the text against some other word. Any word
within a distance of 2 will be matched.

Future work
---
Eventually this will be used as a grepping tool for searching a file directory.
