from vidyut.prakriya import *

v = Vyakarana()
abhibhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi).with_prefixes(["aBi"])
krdanta = Pratipadika.from_krdanta(dhatu=abhibhu, krt=Krt.Rvul)
prakriyas = v.derive_pratipadikas(pratipadika=krdanta)

assert len(prakriyas) == 1
assert prakriyas[0].text == "aBiBAvaka"
print(prakriyas)

