// String utilities module

fn | reverse_string (str | s) > {
  // Reverse a string
  list | chars -> []
  for i in 0..s.len() {
    chars -> chars + [s[i]]
  }
  str | result -> ""
  for i in 0..chars.len() {
    result -> result + chars[chars.len() - 1 - i]
  }
  result
}

fn | repeat_string (str | s, int | n) > {
  str | result -> ""
  for i in 0..n {
    result -> result + s
  }
  result
}

fn | count_vowels (str | s) > {
  int | count -> 0
  list | vowels -> ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"]
  for i in 0..s.len() {
    for v in vowels {
      if s[i] == v {
        count -> count + 1
      }
    }
  }
  count
}
