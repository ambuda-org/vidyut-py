from vidyut.lipi import transliterate, detect, Scheme

text = "1 12 123 1234 12345"
output = transliterate(text, Scheme.HarvardKyoto, Scheme.Grantha)
assert output == "௧ ௰௨ ௱௨௰௩ ௲௨௱௩௰௪ ௰௨௲௩௱௪௰௫"


text = "नरइति"
output = transliterate(text, Scheme.Devanagari, Scheme.Iso15919)
assert output == "nara:iti"
