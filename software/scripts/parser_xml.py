import os
import sys
from pathlib import Path


def main():
  if len(sys.argv) != 2:
    print("Usage: python parser_xml.py [path]")

  fp = sys.argv[1]
  nf = []
  with open(fp, "r") as f:
    t = f.read()
    for l in t.split("\n"):
      ll = l.strip()
      if len(ll) == 0:
        continue
      if "> " not in ll:
        nf.append(ll)
        continue
      a = ll.split("> ")
      nf.append(f"{a[0]}>")
      a = a[1]
      a = a.split(" <")
      nf.append(a[0])
      nf.append(f"<{a[1]}")
  # print(nf)
  nfp = Path(fp)
  s = nfp.stem
  nfp = nfp.with_stem(f"{s}_parsed")
  with open(nfp, "w") as f:
    f.write("\n".join(nf))
if __name__ == "__main__":
  main()
