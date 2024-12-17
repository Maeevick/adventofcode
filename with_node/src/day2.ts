export function part1(content: string[]) {
  return content.reduce((safeReportCount: number, currentReport: string) => {
    if(isSorted(currentReport) && isGapBetweenOneAndThree(currentReport)) {
      return safeReportCount + 1
    }
    return safeReportCount
  }, 0)
}

export function part2(content: string[]) {
  return content.reduce((safeReportCount: number, currentReport: string) => {
    if(isSortedAndGapBetweenOneAndThreeAllowingOneException(currentReport)) {
      return safeReportCount + 1
    }
    return safeReportCount
  }, 0)
}

function isSorted(report: string) {
  const sortedASCReport = report.split(' ').sort((a, b) => Number(a) - Number(b)).join(' ')
  const sortedDESCReport = report.split(' ').sort((a, b) => Number(b) - Number(a)).join(' ')
  
  return report === sortedASCReport || report === sortedDESCReport
}

function isGapBetweenOneAndThree(report: string) {
  return report.split(' ').every((value, index, array) => {
    
    const delta = Math.abs(Number(array[index - 1]) - Number(value))
    return index === 0 || (delta <= 3 && delta >= 1)
  })
}


function isSortedAndGapBetweenOneAndThreeAllowingOneException(report: string) {
  if(isSorted(report) && isGapBetweenOneAndThree(report)) {
    return true
  }

  return report.split(' ').some((_, i,a) => {
    return isSorted(a.filter((_, j) => j !== i).join(' ')) && isGapBetweenOneAndThree(a.filter((_, j) => j !== i).join(' '))
  })
}
