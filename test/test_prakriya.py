from vidyut import Ashtadhyayi, Dhatupatha, Prayoga, Purusha, Vacana, Sanadi, Lakara

a = Ashtadhyayi()


def test_dhatupatha():
    # Path is relative to the project root.
    d = Dhatupatha("test/data/test-dhatupatha.tsv")

    bhu = d["01.0001"]
    assert bhu.upadesha == "BU"

    kr = d["08.0010"]
    assert kr.upadesha == "qukf\\Y"


def test_basic_kartari_tinantas():
    prakriyas = a.derive_tinantas(
        dhatu="BU",
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )
    assert len(prakriyas) == 1
    assert prakriyas[0].text == "Bavati"


def test_basic_karmani_tinantas():
    prakriyas = a.derive_tinantas(
        dhatu="BU",
        prayoga=Prayoga.Karmani,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )
    assert len(prakriyas) == 1
    assert prakriyas[0].text == "BUyate"


def test_sannanta_tinantas():
    prakriyas = a.derive_tinantas(
        dhatu="BU",
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.San,
    )
    assert len(prakriyas) == 1
    assert prakriyas[0].text == "buBUzati"


def test_nijanta_tinantas():
    prakriyas = a.derive_tinantas(
        dhatu="BU",
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.Nic,
    )
    assert len(prakriyas) == 2
    results = {x.text for x in prakriyas}
    assert results == {"BAvayati", "BAvayate"}


def test_yananta_tinantas():
    prakriyas = a.derive_tinantas(
        dhatu="BU",
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.Yan,
    )
    assert len(prakriyas) == 1
    assert prakriyas[0].text == "boBUyate"
