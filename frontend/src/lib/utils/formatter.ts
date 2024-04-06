export const toHHMMSS = (sec_num: number) => {
  if (isNaN(sec_num)) {
    return NaN
  }

  sec_num = Math.floor(sec_num)
  const hours = Math.floor(sec_num / 3600)
  const minutes = Math.floor(sec_num / 60) % 60
  const seconds = sec_num % 60

  return [hours, minutes, seconds]
    .map(v => (v < 10 ? '0' + v : v))
    .filter((v, i) => v !== '00' || i > 0)
    .join(':')
}
