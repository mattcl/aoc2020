run DAY:
  ./scripts/run {{DAY}}

mem DAY:
  ./scripts/mem {{DAY}}

flame DAY:
  ./scripts/flame {{DAY}}

perf DAY: (mem DAY) (flame DAY)
