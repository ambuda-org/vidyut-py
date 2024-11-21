from vidyut.sandhi import Splitter

splitter = Splitter.from_csv("/Users/arun/personal/vidyut/data/build/vidyut-latest/sandhi-rules.csv")

for split in splitter.split_at("ityapi", 2):
    print(split.first, split.second, split.is_valid)

