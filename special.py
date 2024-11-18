from vidyut.prakriya import *

v = Vyakarana()

bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
bhavaka = Pratipadika.from_krdanta(bhu, Krt.Rvul)

guru = Pratipadika(text="guru")
gaurava = Pratipadika.from_taddhitanta(guru, Taddhita.aR)
