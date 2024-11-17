from vidyut.prakriya import *

# dhatupatha = Dhatupatha("/Users/arun/personal/vidyut/data/build/vidyut-latest/prakriya/dhatupatha.tsv")
# dhatus = [e.dhatu for e in dhatupatha.entries()]

bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
bhavaya = bhu.with_sanadi([Sanadi.Ric])
bubhusha = bhu.with_sanadi([Sanadi.san])
bubhushaya = bhu.with_sanadi([Sanadi.san, Sanadi.Ric])
bobhuya = bhu.with_sanadi([Sanadi.yaN])
bobhu = bhu.with_sanadi([Sanadi.yaNluk])

v = Vyakarana()
for dhatu in [bhu, bhavaya, bubhusha, bubhushaya, bobhuya, bobhu]:
    prakriyas = v.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )
    for p in prakriyas:
        print(p.text)
