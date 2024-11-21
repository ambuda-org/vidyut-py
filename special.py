from vidyut.prakriya import (
    Vyakarana,
    Dhatu,
    Gana,
    Prayoga,
    Purusha,
    Vacana,
    Lakara
)

# 1. Create a new `Vyakarana` object.
v = Vyakarana()

# 2. Pass arguments to define our derivation.
bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
prakriyas = v.derive_tinantas(
    dhatu=bhu,
    prayoga=Prayoga.Kartari,
    lakara=Lakara.Lat,
    purusha=Purusha.Prathama,
    vacana=Vacana.Eka,
)

# 3. Receive a list of prakriyas.
for prakriya in prakriyas:
    print(prakriya.text)
    print('===================')
    for step in prakriya.history:
        result = ' + '.join(step.result)
        print("{:<10}: {}".format(step.code, result))
