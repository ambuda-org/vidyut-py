from pathlib import Path

import vidyut.prakriya as vp


# base = Path(os.environ["VIDYUT_DATA_DIR"])
base = Path("/Users/arun/personal/vidyut/data/build/vidyut-latest/")
dhatupatha = vp.Dhatupatha(base / "prakriya" / "dhatupatha.tsv")


v = vp.Vyakarana()

for e in dhatupatha.entries():
    for lakara in vp.Lakara.choices():
        for purusha in vp.Purusha.choices():
            for vacana in vp.Vacana.choices():
                prakriyas = v.derive_tinantas(
                    dhatu=e.dhatu,
                    prayoga=vp.Prayoga.Kartari,
                    purusha=purusha,
                    vacana=vacana,
                    lakara=lakara,
                )
                for p in prakriyas:
                    print(p.text)
