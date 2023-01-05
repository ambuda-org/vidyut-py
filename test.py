from vidyut import Segmenter
import sys

data_path = sys.argv[1]
s = Segmenter(data_path)

text = """sa BagavAnsfzwvedaM jagattasya ca sTitiM cikIrzuH marIcyAdInagre sfzwvA prajApatInpravfttilakzaRaM DarmaM grAhayAmAsa vedoktam"""

print("loaded")
for word in s.segment(text):
    print(word)
