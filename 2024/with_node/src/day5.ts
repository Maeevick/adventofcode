export function part1([orderingRules, updates]: string[][]) {
  console.log('RULES', orderingRules)
  console.log('UPDATES', updates)
  
  const validUpdates = []
  for(const update of updates) {
    const pages = update.split(',')
    for(let i = 1; i < pages.length; i++) {
      const prev = pages[i - 1]
      const next = pages[i]
      
      console.log('PAGE', prev, next)
    }
    console.log('-------')
  }


  return 0
}

export function part2(content: string[]) {
  return 0;
}
