export function part1(content: string[]) {
  const matches = content.join('').matchAll(/mul\((\d{1,3}),(\d{1,3})\)/g) 
  let sum = 0
  for (const [_,a,b] of matches) {
    sum += Number(a) * Number(b)
  }
  return sum
}

export function part2(content: string[]) {
  const matches = content.join('').matchAll(/(don't|do)\(\)|mul\((\d{1,3}),(\d{1,3})\)/g) 
  let processing = true
  let sum = 0
  for (const [_,instruction,a,b] of matches) {
    if(instruction) {
      processing = instruction === 'do'
    }
    if(processing && a && b) {
      sum += Number(a) * Number(b)
    }
  }
  return sum
}
