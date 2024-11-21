from vidyut.chandas import Chandas

c = Chandas(
    "/Users/Arun/personal/vidyut/data/build/vidyut-latest/chandas/data/meters.tsv"
)
match = c.classify("mAtaH samastajagatAM maDukEwaBAreH")

print(match.padya)
print("-" * 20)
for pada in match.aksharas:
    for akshara in pada:
        print(akshara.text, akshara.weight)
