from vidyut.prakriya import *

v = Vyakarana()
guru = Pratipadika(text="guru")

for taddhita in Taddhita.choices():
    prakriyas = v.derive_taddhitantas(pratipadika=guru, taddhita=taddhita)
    for p in prakriyas:
        print(taddhita, p.text)
    else:
        print(f"- (no results for guru + {taddhita})")


