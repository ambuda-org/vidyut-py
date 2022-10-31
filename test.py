from vidyut import Segmenter
import sys

data_path = sys.argv[1]
s = Segmenter(data_path)

print('loaded')
for word in s.segment("Darmakzetre kurukzetre samavetA yuyutsavaH"):
    print(word)
