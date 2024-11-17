from vidyut.lipi import transliterate, detect, Scheme

text = "tapaHsvAdhyAyanirataM tapasvI vAgvidAM varam"
input_scheme = detect(text)

for scheme in Scheme.choices():
    output = transliterate(text, input_scheme, scheme)
    print(f"{scheme}: {output}")
